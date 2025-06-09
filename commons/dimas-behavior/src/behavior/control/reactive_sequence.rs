// Copyright © 2025 Stephan Kunz

//! `ReactiveSequence` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType, error::BehaviorError},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- ReactiveSequence
/// A `ReactiveSequence` ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorState::Failure`] the sequence returns [`BehaviorState::Failure`].
/// - If all children return [`BehaviorState::Success`] the sequence returns [`BehaviorState::Success`].
/// - While any child returns [`BehaviorState::Running`] the sequence returns [`BehaviorState::Running`].
///
/// If all the children return SUCCESS, this node returns SUCCESS.
///
/// IMPORTANT: to work properly, this node should not have more than a single
///            asynchronous child.
#[derive(Behavior, Debug)]
pub struct ReactiveSequence {
	/// Defaults to '-1'
	running_child_idx: i32,
}

impl Default for ReactiveSequence {
	fn default() -> Self {
		Self { running_child_idx: -1 }
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for ReactiveSequence {
	async fn halt(
		&mut self,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		self.running_child_idx = -1;
		children.halt(0, runtime)?;
		Ok(())
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	#[allow(clippy::cast_sign_loss)]
	async fn tick(
		&mut self,
		state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let mut all_skipped = true;
		if state == BehaviorState::Idle {
			self.running_child_idx = -1;
		}

		let children_count = children.len();
		for child_idx in 0..children_count {
			let child = &mut children[child_idx];
			let new_state = child.execute_tick(runtime).await?;

			all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {
					self.running_child_idx = -1;
					children.reset(runtime)?;
					return Ok(BehaviorState::Failure);
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State("ReactiveSequence".into(), "Idle".into()));
				}
				BehaviorState::Running => {
					// halt previously running child
					if self.running_child_idx != (child_idx as i32) && self.running_child_idx != -1 {
						children[self.running_child_idx as usize]
							.execute_halt(runtime)
							.await?;
					}
					if self.running_child_idx == -1 {
						self.running_child_idx = child_idx as i32;
					} else if self.running_child_idx != (child_idx as i32) {
						// Multiple children running at the same time
						return Err(BehaviorError::Composition(
							"[ReactiveSequence]: Only a single child can return Running.".into(),
						));
					}
					return Ok(BehaviorState::Running);
				}
				BehaviorState::Skipped => {
					// halt current child
					child.execute_halt(runtime).await?;
				}
				BehaviorState::Success => {
					self.running_child_idx = -1;
				}
			}
		}

		// Reset children
		// children.reset(runtime)?;

		if all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Success)
		}
	}
}

impl BehaviorStatic for ReactiveSequence {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ReactiveSequence
