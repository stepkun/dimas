// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in if-then-else node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
use tracing::warn;
//endregion:    --- modules

/// IfThenElseNode must have exactly 2 or 3 children. This node is NOT reactive.
///
/// The first child is the "statement" of the if.
///
/// If that return SUCCESS, then the second child is executed.
///
/// Instead, if it returned FAILURE, the third child is executed.
///
/// If you have only 2 children, this node will return FAILURE whenever the
/// statement returns FAILURE.
///
/// This is equivalent to add AlwaysFailure as 3rd child.
#[behavior(SyncControl)]
pub struct IfThenElse {
	#[bhvr(default = "0")]
	child_idx: usize,
}

#[behavior(SyncControl)]
impl IfThenElse {
	async fn tick(&mut self) -> BehaviorResult {
		let children_count = bhvr_.children().len();
		// Node should only have 2 or 3 children
		if !(2..=3).contains(&children_count) {
			return Err(BehaviorError::Composition(
				"IfThenElseNode must have either 2 or 3 children.".to_string(),
			));
		}

		bhvr_.set_status(BehaviorStatus::Running);

		if self.child_idx == 0 {
			let status = bhvr_.children_mut()[0].execute_tick().await?;
			match status {
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Success => self.child_idx += 1,
				BehaviorStatus::Failure => {
					if children_count == 3 {
						self.child_idx = 2;
					} else {
						return Ok(BehaviorStatus::Failure);
					}
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Node name here".to_string(),
						"Idle".to_string(),
					));
				}
				_ => warn!("Condition node of IfThenElseNode returned Skipped"),
			}
		}

		if self.child_idx > 0 {
			let status = bhvr_.children_mut()[self.child_idx]
				.execute_tick()
				.await?;
			match status {
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				status => {
					bhvr_.reset_children().await;
					self.child_idx = 0;
					return Ok(status);
				}
			}
		}

		Err(BehaviorError::Composition(
			"Something unexpected happened in IfThenElseNode".to_string(),
		))
	}

	async fn halt(&mut self) {
		self.child_idx = 0;
		bhvr_.reset_children().await;
	}
}
