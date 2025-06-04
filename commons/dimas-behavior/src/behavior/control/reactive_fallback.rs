// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
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
#[derive(Behavior, Debug, Default)]
pub struct ReactiveFallback {}

#[async_trait::async_trait]
impl BehaviorInstance for ReactiveFallback {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let mut all_skipped = true;

		for index in 0..children.len() {
			let child = &mut children[index];
			let new_state = child.execute_tick(runtime).await?;

			all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {}
				BehaviorState::Idle => {
					return Err(BehaviorError::State(
						"ReactiveFallback".into(),
						"Idle".into(),
					));
				}
				BehaviorState::Running => {
					// stop later children
					for i in 0..index {
						let cd = &mut children[i];
						cd.execute_halt(runtime).await?;
					}
					return Ok(BehaviorState::Running);
				}
				BehaviorState::Skipped => {
					child.execute_halt(runtime).await?;
				}
				BehaviorState::Success => {
					children.reset(runtime)?;
					return Ok(BehaviorState::Success);
				}
			}
		}

		children.reset(runtime)?;

		if all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for ReactiveFallback {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ReactiveFallback
