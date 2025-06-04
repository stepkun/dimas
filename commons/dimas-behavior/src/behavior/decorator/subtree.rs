// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Subtree
/// A `Subtree` is a `Decorator` but with its own [`BehaviorType`].
#[derive(Behavior, Debug, Default)]
pub struct Subtree {}

#[async_trait::async_trait]
impl BehaviorInstance for Subtree {
	async fn halt(
		&mut self,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		children[0].execute_halt(runtime).await
	}

	async fn start(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		children[0].execute_tick(runtime).await
	}

	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		children[0].execute_tick(runtime).await
	}
}

impl BehaviorStatic for Subtree {
	fn kind() -> BehaviorType {
		BehaviorType::SubTree
	}
}
// endregion:   --- Subtree
