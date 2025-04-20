// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString};
use dimas_behavior_derive::Behavior;

use crate::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType, error::NewBehaviorError,
	},
	new_port::NewPortList,
	tree::BehaviorTreeComponent,
};
// endregeion:  --- modules

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
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.tick_data.status = NewBehaviorStatus::Running;

		let child = &mut tree_node.children[0];
		let new_status = child.execute_tick()?;

		match new_status {
			NewBehaviorStatus::Failure => {
				tree_node.reset_child()?;
				Ok(NewBehaviorStatus::Failure)
			}
			NewBehaviorStatus::Idle => Err(NewBehaviorError::Status(
				"Inverter".to_string(),
				"Idle".to_string(),
			)),
			status @ (NewBehaviorStatus::Running | NewBehaviorStatus::Skipped) => Ok(status),
			NewBehaviorStatus::Success => {
				tree_node.reset_child()?;
				Ok(NewBehaviorStatus::Failure)
			}
		}
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorStaticMethods for Inverter {
	fn kind() -> NewBehaviorType {
		NewBehaviorType::Decorator
	}
}
// endregion:   --- Inverter
