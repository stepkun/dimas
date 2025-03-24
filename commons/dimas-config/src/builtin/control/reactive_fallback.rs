// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in reactive-fallback node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The ReactiveFallback is similar to a ParallelNode.
/// All the children are ticked from first to last:
///
/// - If a child returns RUNNING, continue to the next sibling.
/// - If a child returns FAILURE, continue to the next sibling.
/// - If a child returns SUCCESS, stop and return SUCCESS.
///
/// If all the children fail, than this node returns FAILURE.
///
/// IMPORTANT: to work properly, this node should not have more than
///            a single asynchronous child.
#[behavior(SyncControl)]
pub struct ReactiveFallback {}

#[behavior(SyncControl)]
impl ReactiveFallback {
	async fn tick(&mut self) -> BehaviorResult {
		let mut all_skipped = true;
		bhvr_.set_status(BehaviorStatus::Running);

		for index in 0..bhvr_.children().len() {
			let cur_child = &mut bhvr_.children_mut()[index];

			let child_status = cur_child.execute_tick().await?;

			all_skipped &= child_status == BehaviorStatus::Skipped;

			match &child_status {
				BehaviorStatus::Running => {
					for i in 0..index {
						bhvr_.halt_child_idx(i).await?;
					}

					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Failure => {}
				BehaviorStatus::Success => {
					bhvr_.reset_children().await;
					return Ok(BehaviorStatus::Success);
				}
				BehaviorStatus::Skipped => {
					bhvr_.halt_child_idx(index).await?;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Name here".to_string(),
						"Idle".to_string(),
					));
				}
			}
		}

		bhvr_.reset_children().await;

		if all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}

	async fn halt(&mut self) {
		bhvr_.reset_children().await;
	}
}
