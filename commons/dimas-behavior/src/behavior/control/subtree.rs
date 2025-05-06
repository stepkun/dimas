// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		error::BehaviorError, BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods, BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTickData, BehaviorTreeMethods, BehaviorType
	}, blackboard::BlackboardNodeRef, port::PortList, tree::{BehaviorTreeComponent, BehaviorTreeComponentList}
};
// endregion:   --- modules

// region:      --- Subtree
/// A `Subtree` ticks executes children in
///
#[derive(Behavior, Debug, Default)]
pub struct Subtree {}

impl BehaviorInstanceMethods for Subtree {
	fn halt(&mut self, children: &mut BehaviorTreeComponentList) -> Result<(), BehaviorError> {
		children[0].execute_halt()
	}

	fn start(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut BlackboardNodeRef,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		children[0].execute_tick()
	}

	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut BlackboardNodeRef,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		children[0].execute_tick()
	}
}

impl BehaviorStaticMethods for Subtree {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Subtree
