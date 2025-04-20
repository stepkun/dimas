// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Behaviors for benchmarks

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType,
	},
	new_port::NewPortList,
	tree::BehaviorTreeComponent,
};
use dimas_behavior_derive::Behavior;

/// Action `AlwaysSuccess`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysSuccess {}

impl BehaviorInstanceMethods for AlwaysSuccess {
	fn tick(&mut self, _tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorCreationMethods for AlwaysSuccess {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorStaticMethods for AlwaysSuccess {}

/// Action `AlwaysFailure`
#[derive(Behavior, Debug, Default)]
pub struct AlwaysFailure {}

impl BehaviorInstanceMethods for AlwaysFailure {
	fn tick(&mut self, _tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorCreationMethods for AlwaysFailure {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorStaticMethods for AlwaysFailure {}
