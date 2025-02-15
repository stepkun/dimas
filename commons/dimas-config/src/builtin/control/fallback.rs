// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in fallback node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The FallbackNode is used to try different strategies,
/// until one succeeds.
/// If any child returns RUNNING, previous children will NOT be ticked again.
///
/// - If all the children return FAILURE, this node returns FAILURE.
///
/// - If a child returns RUNNING, this node returns RUNNING.
///
/// - If a child returns SUCCESS, stop the loop and return SUCCESS.
#[behavior(SyncControl)]
pub struct Fallback {
	#[bhvr(default = "0")]
	child_idx: usize,
	#[bhvr(default = "true")]
	all_skipped: bool,
}

#[behavior(SyncControl)]
impl Fallback {
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
				BehaviorStatus::Running => {
					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Failure | BehaviorStatus::Skipped => {
					self.child_idx += 1;
				}
				BehaviorStatus::Success => {
					bhvr_.reset_children().await;
					self.child_idx = 0;
					return Ok(BehaviorStatus::Success);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Name here".to_string(),
						"Idle".to_string(),
					));
				}
			};
		}

		if self.child_idx == bhvr_.children().len() {
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
