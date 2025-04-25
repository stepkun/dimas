// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		error::BehaviorError, BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods, BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus, BehaviorTickData, BehaviorTreeMethods, BehaviorType
	},
	port::PortList,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentList},
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

extern crate std;
impl BehaviorInstanceMethods for Inverter {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		tick_data.status = BehaviorStatus::Running;

		let child = &mut children[0];
		let new_status = child.execute_tick()?;

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

impl BehaviorStaticMethods for Inverter {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}
}
// endregion:   --- Inverter
