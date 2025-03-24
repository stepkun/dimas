// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in sequence node of `DiMAS`

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The Sequence is used to tick children in an ordered sequence.
/// - If any child returns RUNNING, previous children will NOT be ticked again.
/// - If all the children return SUCCESS, this node returns SUCCESS.
/// - If a child returns RUNNING, this node returns RUNNING.
///   Loop is NOT restarted, the same running child will be ticked again.
/// - If a child returns FAILURE, stop the loop and return FAILURE.
#[behavior(SyncControl)]
pub struct Sequence {
	#[bhvr(default = "0")]
	child_idx: usize,
	#[bhvr(default = "false")]
	all_skipped: bool,
}

#[behavior(SyncControl)]
impl Sequence {
	async fn tick(&mut self) -> BehaviorResult {
		if bhvr_.status() == BehaviorStatus::Idle {
			self.all_skipped = true;
		}

		bhvr_.set_status(BehaviorStatus::Running);

		while self.child_idx < bhvr_.children().len() {
			let cur_child = &mut bhvr_.children_mut()[self.child_idx];

			let _prev_status = cur_child.status();
			let child_status = cur_child.execute_tick().await?;

			self.all_skipped &= child_status == BehaviorStatus::Skipped;

			match &child_status {
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Failure => {
					bhvr_.reset_children().await;
					self.child_idx = 0;
					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Success | BehaviorStatus::Skipped => {
					self.child_idx += 1;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"SequenceNode".to_string(),
						"Idle".to_string(),
					));
				}
			}
		}

		if self.child_idx == bhvr_.children().len() {
			bhvr_.reset_children().await;
			self.child_idx = 0;
		}

		Ok(BehaviorStatus::Success)
	}

	async fn halt(&mut self) {
		self.child_idx = 0;
		bhvr_.reset_children().await;
	}
}
