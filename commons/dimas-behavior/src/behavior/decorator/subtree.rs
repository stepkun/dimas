// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorTickData, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentList},
};
// endregion:   --- modules

// region:      --- Subtree
/// A `Subtree` is a `Decorator` but with its own [`BehaviorType`].
#[derive(Behavior, Debug, Default)]
pub struct Subtree {}

impl BehaviorInstance for Subtree {
	fn halt(&mut self, children: &mut BehaviorTreeComponentList) -> Result<(), BehaviorError> {
		children[0].execute_halt()
	}

	fn start(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		children[0].execute_tick()
	}

	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		children[0].execute_tick()
	}
}

impl BehaviorStatic for Subtree {
	fn kind() -> BehaviorType {
		BehaviorType::SubTree
	}
}
// endregion:   --- Subtree
