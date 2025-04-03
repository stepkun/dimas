// Copyright © 2025 Stephan Kunz

//! Built in scripted action behavior of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The Script behavior returns Success or Failure depending on the result of the scripted code
#[behavior(SyncAction)]
pub struct Script {}

#[behavior(SyncAction)]
impl Script {
	async fn tick(&mut self) -> BehaviorResult {
		bhvr_.set_status(BehaviorStatus::Running);

		// @TODO: Implement

		Ok(BehaviorStatus::Success)
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
