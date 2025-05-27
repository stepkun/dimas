// Copyright © 2025 Stephan Kunz

//! `ParallelAll` behavior implementation
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

// region:      --- ParallelAll
/// A `ParallelAll` executes its children
///
#[derive(Behavior, Debug, Default)]
pub struct ParallelAll {}

#[async_trait::async_trait]
impl BehaviorInstance for ParallelAll {
	async fn halt(&mut self, children: &mut BehaviorTreeElementList) -> Result<(), BehaviorError> {
		children.halt(0)
	}

	async fn tick(
		&mut self,
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for ParallelAll {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ParallelAll
