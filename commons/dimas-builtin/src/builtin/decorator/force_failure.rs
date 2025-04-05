// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in force-failure node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The ForceFailureNode returns always Failure or Running
#[behavior(SyncDecorator)]
pub struct ForceFailure {}

#[behavior(SyncDecorator)]
impl ForceFailure {
	async fn tick(&mut self) -> BehaviorResult {
		bhvr_.set_status(BehaviorStatus::Running);

		let child_status = bhvr_
			.child()
			.unwrap_or_else(|| todo!())
			.execute_tick()
			.await?;

		if child_status.is_completed() {
			bhvr_.reset_child().await;

			return Ok(BehaviorStatus::Failure);
		}

		Ok(child_status)
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
