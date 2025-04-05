// Copyright © 2025 Stephan Kunz

//! Built in scripted condition behavior of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The ScriptCondition returns Success or Failure depending on the result of the scripted code
#[behavior(SyncCondition)]
pub struct ScriptCondition {}

#[behavior(SyncCondition)]
impl ScriptCondition {
	async fn tick(&mut self) -> BehaviorResult {
		bhvr_.set_status(BehaviorStatus::Running);

		// @TODO: Implement

		Ok(BehaviorStatus::Success)
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
