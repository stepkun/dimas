// Copyright © 2025 Stephan Kunz

//! `ParallelAll` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_behavior_derive::Behavior;

use crate::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType,
	},
	new_port::NewPortList,
	tree::BehaviorTreeComponent,
};
// endregeion:  --- modules

// region:      --- ParallelAll
/// A `ParallelAll` executes its children
///
#[derive(Behavior, Debug, Default)]
pub struct ParallelAll {}

impl BehaviorInstanceMethods for ParallelAll {
	fn tick(&mut self, _tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		Ok(NewBehaviorStatus::Failure)
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorCreationMethods for ParallelAll {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for ParallelAll {}
// endregion:   --- ParallelAll
