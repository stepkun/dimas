// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Behaviors for benchmarks

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType,
	},
	port::PortList,
	tree::BehaviorTreeComponentList,
};
use dimas_behavior_derive::Behavior;

/// Action `AlwaysSuccess`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysSuccess {}

impl BehaviorInstanceMethods for AlwaysSuccess {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for AlwaysSuccess {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

/// Action `AlwaysFailure`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysFailure {}

impl BehaviorInstanceMethods for AlwaysFailure {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for AlwaysFailure {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}
