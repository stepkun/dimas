// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString};
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType, error::BehaviorError,
	},
	port::PortList,
	tree::BehaviorTreeComponentList,
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

impl BehaviorInstanceMethods for ReactiveFallback {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let mut all_skipped = true;
		tick_data.status = BehaviorStatus::Running;

		for index in 0..children.len() {
			let child = &mut children[index];
			let new_status = child.execute_tick()?;

			all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"ReactiveFallback".to_string(),
						"Idle".to_string(),
					));
				}
				BehaviorStatus::Running => {
					// stop later children
					for i in 0..index {
						let cd = &mut children[i];
						cd.execute_halt()?;
					}
					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Skipped => {
					child.execute_halt()?;
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

impl BehaviorStaticMethods for ReactiveFallback {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ReactiveFallback
