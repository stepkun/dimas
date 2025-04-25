// Copyright © 2025 Stephan Kunz

//! `ParallelAll` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType, error::BehaviorError,
	},
	port::PortList,
	tree::BehaviorTreeComponentList,
};
// endregion:   --- modules

// region:      --- ParallelAll
/// A `ParallelAll` executes its children
///
#[derive(Behavior, Debug, Default)]
pub struct ParallelAll {}

impl BehaviorInstanceMethods for ParallelAll {
	fn halt(&mut self, children: &mut BehaviorTreeComponentList) -> Result<(), BehaviorError> {
		children.halt(0)
	}

	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for ParallelAll {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ParallelAll
