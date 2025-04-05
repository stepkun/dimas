// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in inverter node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_behavior::behavior::error::BehaviorError;
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::behavior;
//endregion:    --- modules

/// The InverterNode returns Failure on Success, and Success on Failure
#[behavior(SyncDecorator)]
pub struct Inverter {}

#[behavior(SyncDecorator)]
impl Inverter {
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
				Ok(BehaviorStatus::Failure)
			}
			BehaviorStatus::Failure => {
				bhvr_.reset_child().await;
				Ok(BehaviorStatus::Success)
			}
			status @ (BehaviorStatus::Running | BehaviorStatus::Skipped) => Ok(status),
			BehaviorStatus::Idle => Err(BehaviorError::Status(
				"Inverter Decorator".to_string(),
				"Idle".to_string(),
			)),
		}
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
