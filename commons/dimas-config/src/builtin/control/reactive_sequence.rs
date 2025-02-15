// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in reactive-sequence node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The ReactiveSequence is similar to a ParallelNode.
/// All the children are ticked from first to last:
///
/// - If a child returns RUNNING, halt the remaining siblings in the sequence and return RUNNING.
/// - If a child returns SUCCESS, tick the next sibling.
/// - If a child returns FAILURE, stop and return FAILURE.
///
/// If all the children return SUCCESS, this node returns SUCCESS.
///
/// IMPORTANT: to work properly, this node should not have more than a single
///            asynchronous child.
#[behavior(SyncControl)]
pub struct ReactiveSequence {
	#[bhvr(default = "-1")]
	running_child: i32,
}

#[behavior(SyncControl)]
impl ReactiveSequence {
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn tick(&mut self) -> BehaviorResult {
		let mut all_skipped = true;

		bhvr_.set_status(BehaviorStatus::Running);

		for counter in 0..bhvr_.children().len() {
			let child = &mut bhvr_.children_mut()[counter];
			let child_status = child.execute_tick().await?;

			all_skipped &= child_status == BehaviorStatus::Skipped;

			match child_status {
				BehaviorStatus::Running => {
					for i in 0..counter {
						bhvr_.children_mut()[i].halt().await;
						// bhvr_.halt_child(i).await?;
					}
					if self.running_child == -1 {
						self.running_child = counter as i32;
					} else if self.running_child != counter as i32 {
						// Multiple children running at the same time
						return Err(BehaviorError::Composition(
							"[ReactiveSequence]: Only a single child can return Running."
								.to_string(),
						));
					}
					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Failure => {
					bhvr_.reset_children().await;
					return Ok(BehaviorStatus::Failure);
				}
				// Do nothing on Success
				BehaviorStatus::Success => {}
				BehaviorStatus::Skipped => {
					// Halt current child
					bhvr_.children_mut()[counter].halt().await;
					// bhvr_.halt_child(counter).await?;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"ReactiveSequenceNode".into(),
						"Idle".to_string(),
					));
				}
			}
		}

		bhvr_.reset_children().await;

		if all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Success)
		}
	}

	async fn halt(&mut self) {
		bhvr_.reset_children().await;
	}
}
