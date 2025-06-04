// Copyright © 2025 Stephan Kunz

//! `ReactiveSequence` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
		error::BehaviorError,
	},
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
#[derive(Behavior, Debug, Default)]
pub struct ReactiveSequence {
	/// Defaults to 'false'
	running: bool,
	/// Defaults to '0'
	child_idx: usize,
}

#[async_trait::async_trait]
impl BehaviorInstance for ReactiveSequence {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let mut all_skipped = true;

		for counter in 0..children.len() {
			let child = &mut children[counter];
			let new_state = child.execute_tick(runtime).await?;

			all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {
					children.reset(runtime)?;
					return Ok(BehaviorState::Failure);
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State(
						"ReactiveSequence".into(),
						"Idle".into(),
					));
				}
				BehaviorState::Running => {
					for i in 0..counter {
						children[i].execute_halt(runtime).await?;
					}
					if !self.running {
						self.child_idx = counter;
						self.running = true;
					} else if self.child_idx != counter {
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
				BehaviorState::Success => {}
			}
		}

		// Reset children on failure
		children.reset(runtime)?;
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
