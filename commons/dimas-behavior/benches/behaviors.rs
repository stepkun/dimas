// Copyright © 2025 Stephan Kunz

//! Behaviors for benchmarks

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};

/// Action `AlwaysSuccess`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysSuccess {}

#[async_trait::async_trait]
impl BehaviorInstance for AlwaysSuccess {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		Ok(BehaviorState::Success)
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

#[async_trait::async_trait]
impl BehaviorInstance for AlwaysFailure {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		Ok(BehaviorState::Failure)
	}
}

impl BehaviorStatic for AlwaysFailure {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}
