// Copyright © 2025 Stephan Kunz

//! Behaviors for benchmarks

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeComponentList,
};
use dimas_behavior_derive::Behavior;

/// Action `AlwaysSuccess`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysSuccess {}

impl BehaviorInstance for AlwaysSuccess {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for AlwaysSuccess {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

/// Action `AlwaysFailure`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysFailure {}

impl BehaviorInstance for AlwaysFailure {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for AlwaysFailure {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}
