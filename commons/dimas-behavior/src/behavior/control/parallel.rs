// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use core::any::Any;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType,
	},
	blackboard::SharedBlackboard,
	port::PortList,
	tree::BehaviorTreeComponentList,
};
// endregion:   --- modules

// region:      --- Parallel
/// A `Parallel` ticks executes children in
///
#[derive(Behavior, Debug, Default)]
pub struct Parallel {}

impl BehaviorInstanceMethods for Parallel {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for Parallel {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Parallel
