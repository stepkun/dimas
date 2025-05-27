// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorType,
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
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		let mut all_skipped = true;

		for index in 0..children.len() {
			let child = &mut children[index];
			let new_status = child.execute_tick().await?;

			all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"ReactiveFallback".into(),
						"Idle".into(),
					));
				}
				BehaviorStatus::Running => {
					// stop later children
					for i in 0..index {
						let cd = &mut children[i];
						cd.execute_halt().await?;
					}
					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Skipped => {
					child.execute_halt().await?;
				}
				BehaviorStatus::Success => {
					children.reset()?;
					return Ok(BehaviorStatus::Success);
				}
			}
		}

		children.reset()?;

		if all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}
}

impl BehaviorStatic for ReactiveFallback {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ReactiveFallback
