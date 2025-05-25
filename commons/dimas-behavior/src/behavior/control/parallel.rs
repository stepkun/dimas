// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Parallel
/// A `Parallel` ticks executes children in
///
#[derive(Behavior, Debug, Default)]
pub struct Parallel {}

impl BehaviorInstance for Parallel {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for Parallel {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Parallel
