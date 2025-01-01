// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in keep-running-until node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The KeepRunningUntilFailureNode returns always Failure or Running
#[behavior(SyncDecorator)]
pub struct KeepRunningUntilFailure {}

#[behavior(SyncDecorator)]
impl KeepRunningUntilFailure {
	async fn tick(&mut self) -> BehaviorResult {
		bhvr_.set_status(BehaviorStatus::Running);

		let child_status = bhvr_
			.child()
			.unwrap_or_else(|| todo!())
			.execute_tick()
			.await?;

		match child_status {
			BehaviorStatus::Success => {
				bhvr_.reset_child().await;
				Ok(BehaviorStatus::Running)
			}
			BehaviorStatus::Failure => {
				bhvr_.reset_child().await;
				Ok(BehaviorStatus::Failure)
			}
			_ => Ok(BehaviorStatus::Running),
		}
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
