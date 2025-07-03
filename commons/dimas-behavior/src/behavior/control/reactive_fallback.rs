// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- ReactiveFallback
/// The `ReactiveFallback` behavior is used to try different strategies until one succeeds,
/// but every strategy is re-evaluated on each tick.
/// All the children are ticked from first to last:
/// - If a child returns RUNNING, continue to the next sibling.
/// - If a child returns FAILURE, continue to the next sibling.
/// - If a child returns SUCCESS, stop and return SUCCESS.
///
/// If all the children fail, than this node returns FAILURE.
///
/// IMPORTANT: to work properly, this node should not have more than
///            a single asynchronous child.
#[derive(Behavior, Debug)]
pub struct ReactiveFallback {
	/// Defaults to '-1'
	running_child_idx: i32,
}

impl Default for ReactiveFallback {
	fn default() -> Self {
		Self { running_child_idx: -1 }
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for ReactiveFallback {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		self.running_child_idx = -1;
		children.reset(runtime).await?;
		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	#[allow(clippy::cast_sign_loss)]
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let mut all_skipped = true;
		self.running_child_idx = -1;

		behavior.set_state(BehaviorState::Running);

		for child_idx in 0..children.len() {
			let child = &mut children[child_idx];
			let new_state = child.execute_tick(runtime).await?;

			all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {
					self.running_child_idx = -1;
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State("ReactiveFallback".into(), "Idle".into()));
				}
				BehaviorState::Running => {
					// halt previously running child
					if self.running_child_idx != (child_idx as i32) && self.running_child_idx != -1 {
						children[self.running_child_idx as usize]
							.execute_halt(runtime)
							.await?;
					}
					self.running_child_idx = child_idx as i32;
					if self.running_child_idx == -1 {
						self.running_child_idx = child_idx as i32;
					} else if self.running_child_idx != (child_idx as i32) {
						// Multiple children running at the same time
						return Err(BehaviorError::Composition(
							"[ReactiveFallback]: Only a single child can return Running.".into(),
						));
					}
					return Ok(BehaviorState::Running);
				}
				BehaviorState::Skipped => {
					child.execute_halt(runtime).await?;
				}
				BehaviorState::Success => {
					children.reset(runtime).await?;
					self.running_child_idx = -1;
					return Ok(BehaviorState::Success);
				}
			}
		}

		children.reset(runtime).await?;
		self.running_child_idx = -1;

		if all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for ReactiveFallback {
	fn kind() -> BehaviorKind {
		BehaviorKind::Control
	}
}
// endregion:   --- ReactiveFallback
