// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in while-do-else node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_behavior::behavior::error::BehaviorError;
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// WhileDoElse must have exactly 2 or 3 children.
/// It is a REACTIVE node of IfThenElseNode.
///
/// The first child is the "statement" that is executed at each tick
///
/// If result is SUCCESS, the second child is executed.
///
/// If result is FAILURE, the third child is executed.
///
/// If the 2nd or 3d child is RUNNING and the statement changes,
/// the RUNNING child will be stopped before starting the sibling.
#[behavior(SyncControl)]
pub struct WhileDoElse {}

#[behavior(SyncControl)]
impl WhileDoElse {
	async fn tick(&mut self) -> BehaviorResult {
		let children_count = bhvr_.children().len();
		// Node should only have 2 or 3 children
		if !(2..=3).contains(&children_count) {
			return Err(BehaviorError::Composition(
				"IfThenElseNode must have either 2 or 3 children.".to_string(),
			));
		}

		bhvr_.set_status(BehaviorStatus::Running);

		let condition_status = bhvr_.children_mut()[0].execute_tick().await?;

		if matches!(condition_status, BehaviorStatus::Running) {
			return Ok(BehaviorStatus::Running);
		}

		let mut status = BehaviorStatus::Idle;

		match condition_status {
			BehaviorStatus::Success => {
				if children_count == 3 {
					bhvr_.halt_child_idx(2).await?;
				}

				status = bhvr_.children_mut()[1].execute_tick().await?;
			}
			BehaviorStatus::Failure => match children_count {
				3 => {
					bhvr_.halt_child_idx(1).await?;
					status = bhvr_.children_mut()[2].execute_tick().await?;
				}
				2 => {
					status = BehaviorStatus::Failure;
				}
				_ => {}
			},
			_ => {}
		}

		match status {
			BehaviorStatus::Running => Ok(BehaviorStatus::Running),
			status => {
				bhvr_.reset_children().await;
				Ok(status)
			}
		}
	}

	async fn halt(&mut self) {
		bhvr_.reset_children().await;
	}
}
