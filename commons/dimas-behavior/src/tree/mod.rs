// Copyright © 2025 Stephan Kunz

//! [`BehaviorTree`] library
//!

pub mod error;
#[allow(clippy::module_inception)]
mod tree;
mod tree_leaf;
mod tree_node;
mod tree_proxy;

// flatten
pub use tree::{BehaviorTree, BehaviorTreeComponentList, print_tree};
pub use tree_leaf::BehaviorTreeLeaf;
pub use tree_node::BehaviorTreeNode;
pub use tree_proxy::BehaviorTreeProxy;

// region:      --- modules
use alloc::sync::Arc;
use core::any::Any;
use parking_lot::Mutex;

use crate::{
	behavior::{BehaviorResult, error::BehaviorError},
	blackboard::Blackboard,
};
// endregion:   --- modules

//  region:		--- types
/// Shorthand for a behavior subtree definition
/// An `Arc` with `Mutex` to enable reusability in the tree.
pub type BehaviorSubTree = Arc<Mutex<dyn BehaviorTreeComponent>>;
// endregion:	--- types

// region:      --- BehaviorTreeComponent
/// Interface for an element in a [`BehaviorTree`]
pub trait BehaviorTreeComponent: Send + Sync {
	/// Get the id
	fn id(&self) -> &str;

	/// Get the blackboard
	fn blackboard(&self) -> Blackboard;

	/// Get the children
	fn children(&self) -> &BehaviorTreeComponentList;

	/// Get the children mutable
	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList;

	/// Halt the component
	/// # Errors
	fn execute_halt(&mut self) -> Result<(), BehaviorError> {
		self.halt(0)
	}

	/// Tick the component
	/// # Errors
	fn execute_tick(&mut self) -> BehaviorResult;

	/// halt child at `index`
	/// # Errors
	/// - if index is out of childrens bounds
	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError>;

	/// halt all children at and beyond `index`
	/// # Errors
	/// - if index is out of childrens bounds
	fn halt(&mut self, index: usize) -> Result<(), BehaviorError>;

	/// Reset all children for single child components.
	/// # Errors
	fn reset_child(&mut self) -> Result<(), BehaviorError> {
		self.halt_child(0)
	}

	/// Reset all children for multi child components.
	/// # Errors
	fn reset_children(&mut self) -> Result<(), BehaviorError> {
		self.halt(0)
	}

	/// Convert to any
	fn as_any(&self) -> &dyn Any;

	/// Convert to a mutable any
	fn as_any_mut(&mut self) -> &mut dyn Any;
}
// endregion:   --- BehaviorTreeComponent
