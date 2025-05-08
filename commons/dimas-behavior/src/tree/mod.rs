// Copyright © 2025 Stephan Kunz

//! [`BehaviorTree`] library
//!

pub mod error;
#[allow(clippy::module_inception)]
mod tree;
mod tree_component_list;
mod tree_leaf;
mod tree_node;

// flatten
pub use tree::{BehaviorTree, print_tree};
pub use tree_component_list::BehaviorTreeComponentList;
pub use tree_leaf::BehaviorTreeLeaf;
pub use tree_node::BehaviorTreeNode;

// region:      --- modules
use crate::{
	behavior::{BehaviorResult, error::BehaviorError},
	blackboard::SharedBlackboard,
};
// endregion:   --- modules

// region:		--- TreeElement
/// An enum with the different types of tree elements.
///
/// Using an enum makes sense, because there will most likely never be more than two kinds of elements.
/// And it is not intended to give external access to the tree element type system.
pub enum TreeElement {
	/// A final tree leaf
	Leaf(BehaviorTreeLeaf),
	/// An intermediate tree node
	Node(BehaviorTreeNode),
}

impl BehaviorTreeComponent for TreeElement {
	fn id(&self) -> &str {
		match self {
			Self::Leaf(leaf) => leaf.id(),
			Self::Node(node) => node.id(),
		}
	}

	fn path(&self) -> &str {
		match self {
			Self::Leaf(leaf) => leaf.path(),
			Self::Node(node) => node.path(),
		}
	}

	fn blackboard(&self) -> SharedBlackboard {
		match self {
			Self::Leaf(leaf) => leaf.blackboard(),
			Self::Node(node) => node.blackboard(),
		}
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		match self {
			Self::Leaf(leaf) => leaf.children(),
			Self::Node(node) => node.children(),
		}
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList {
		match self {
			Self::Leaf(leaf) => leaf.children_mut(),
			Self::Node(node) => node.children_mut(),
		}
	}

	fn execute_halt(&mut self) -> Result<(), BehaviorError> {
		match self {
			Self::Leaf(leaf) => leaf.execute_halt(),
			Self::Node(node) => node.execute_halt(),
		}
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		match self {
			Self::Leaf(leaf) => leaf.execute_tick(),
			Self::Node(node) => node.execute_tick(),
		}
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		match self {
			// A leaf has no children, so we can return early.
			Self::Leaf(_) => Ok(()),
			Self::Node(node) => node.halt_child(index),
		}
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		match self {
			// A leaf has no children, so we can return early.
			Self::Leaf(_) => Ok(()),
			Self::Node(node) => node.halt(index),
		}
	}
}
// endregion:	--- TreeElement

// region:      --- BehaviorTreeComponent
/// Interface for an element in a [`BehaviorTree`]
pub trait BehaviorTreeComponent: Send + Sync {
	/// Get the id
	fn id(&self) -> &str;

	/// Get the path
	fn path(&self) -> &str;

	/// Get the blackboard
	fn blackboard(&self) -> SharedBlackboard;

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

	/// Reset child at `index`.
	/// # Errors
	/// - if index is out of childrens bounds
	fn reset_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.halt_child(index)
	}

	/// Reset all children.
	/// # Errors
	fn reset_children(&mut self) -> Result<(), BehaviorError> {
		self.halt(0)
	}
}
// endregion:   --- BehaviorTreeComponent
