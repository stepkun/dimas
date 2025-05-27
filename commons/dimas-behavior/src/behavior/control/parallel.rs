// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorType},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Parallel
/// A `Parallel` ticks executes children in
///
#[derive(Behavior, Debug, Default)]
pub struct Parallel {}

#[async_trait::async_trait]
impl BehaviorInstance for Parallel {
	async fn tick(
		&mut self,
		_status: BehaviorStatus,
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
