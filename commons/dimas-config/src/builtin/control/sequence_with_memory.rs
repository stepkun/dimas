// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in sequence-star node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The SequenceWithMemory is used to tick children in an ordered sequence.
/// If any child returns RUNNING, previous children are not ticked again.
///
/// - If all the children return SUCCESS, this node returns SUCCESS.
///
/// - If a child returns RUNNING, this node returns RUNNING.
///   Loop is NOT restarted, the same running child will be ticked again.
///
/// - If a child returns FAILURE, stop the loop and return FAILURE.
///   Loop is NOT restarted, the same running child will be ticked again.
#[behavior(SyncControl)]
pub struct SequenceWithMemory {
	#[bhvr(default = "0")]
	child_idx: usize,
	#[bhvr(default = "false")]
	all_skipped: bool,
}

#[behavior(SyncControl)]
impl SequenceWithMemory {
	async fn tick(&mut self) -> BehaviorResult {
		if bhvr_.status == BehaviorStatus::Idle {
			self.all_skipped = true;
		}

		bhvr_.status = BehaviorStatus::Running;

		while self.child_idx < bhvr_.children.len() {
			let cur_child = &mut bhvr_.children[self.child_idx];

			let _prev_status = cur_child.status();
			let child_status = cur_child.execute_tick().await?;

			self.all_skipped &= child_status == BehaviorStatus::Skipped;

			match &child_status {
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Failure => {
					// Do NOT reset child_idx on failure
					// Halt children at and after this index
					bhvr_.halt_children(self.child_idx).await?;

					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Success | BehaviorStatus::Skipped => {
					self.child_idx += 1;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"SequenceStarNode".to_string(),
						"Idle".to_string(),
					))
				}
			};
		}

		// All children returned Success
		if self.child_idx == bhvr_.children.len() {
			bhvr_.reset_children().await;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}

	async fn halt(&mut self) {
		self.child_idx = 0;
		bhvr_.reset_children().await;
	}
}
