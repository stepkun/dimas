// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Inverter
/// The `Inverter` behavior is used to try different strategies until one succeeds.
/// If any child returns RUNNING, previous children will NOT be ticked again.
/// - If all the children return FAILURE, this node returns FAILURE.
/// - If a child returns RUNNING, this node returns RUNNING.
/// - If a child returns SUCCESS, stop the loop and return SUCCESS.
#[derive(Behavior, Debug, Default)]
pub struct Inverter {}

#[async_trait::async_trait]
impl BehaviorInstance for Inverter {
	async fn tick(
		&mut self,
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		let child = &mut children[0];
		let new_status = child.execute_tick().await?;

		match new_status {
			BehaviorStatus::Failure => {
				children.reset()?;
				Ok(BehaviorStatus::Success)
			}
			BehaviorStatus::Idle => Err(BehaviorError::Status("Inverter".into(), "Idle".into())),
			status @ (BehaviorStatus::Running | BehaviorStatus::Skipped) => Ok(status),
			BehaviorStatus::Success => {
				children.reset()?;
				Ok(BehaviorStatus::Failure)
			}
		}
	}
}

impl BehaviorStatic for Inverter {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}
}
// endregion:   --- Inverter
