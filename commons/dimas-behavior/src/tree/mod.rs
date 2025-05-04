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
use parking_lot::RwLock;

use crate::{
	behavior::{BehaviorResult, error::BehaviorError},
	blackboard::BlackboardNodeRef,
};
// endregion:   --- modules

//  region:		--- types
/// Shorthand for a behavior subtree definition
/// An `Arc` with `Mutex` to enable reusability in the tree.
pub type BehaviorSubTree = Arc<RwLock<TreeElement>>;
// endregion:	--- types

// region:		--- TreeElement
/// An enum with the different types of tree elements.
///
/// Using an enum makes sense, because there will most likely never be more than three kinds of elements.
/// And it is not intended to give external access to the tree element type system.
pub enum TreeElement {
	/// A tree leaf
	Leaf(BehaviorTreeLeaf),
	/// An intermediate tree node
	Node(BehaviorTreeNode),
	/// A connector to subtrees
	Proxy(BehaviorTreeProxy),
}

impl BehaviorTreeComponent for TreeElement {
	fn id(&self) -> &str {
		match self {
			Self::Leaf(leaf) => leaf.id(),
			Self::Node(node) => node.id(),
			Self::Proxy(proxy) => proxy.id(),
		}
	}

	fn blackboard(&self) -> BlackboardNodeRef {
		match self {
			Self::Leaf(leaf) => leaf.blackboard(),
			Self::Node(node) => node.blackboard(),
			Self::Proxy(proxy) => proxy.blackboard(),
		}
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		match self {
			Self::Leaf(leaf) => leaf.children(),
			Self::Node(node) => node.children(),
			Self::Proxy(proxy) => proxy.children(),
		}
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList {
		match self {
			Self::Leaf(leaf) => leaf.children_mut(),
			Self::Node(node) => node.children_mut(),
			Self::Proxy(proxy) => proxy.children_mut(),
		}
	}

	fn execute_halt(&mut self) -> Result<(), BehaviorError> {
		match self {
			Self::Leaf(leaf) => leaf.execute_halt(),
			Self::Node(node) => node.execute_halt(),
			Self::Proxy(proxy) => proxy.execute_halt(),
		}
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		match self {
			Self::Leaf(leaf) => leaf.execute_tick(),
			Self::Node(node) => node.execute_tick(),
			Self::Proxy(proxy) => proxy.execute_tick(),
		}
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		match self {
			Self::Leaf(leaf) => leaf.halt_child(index),
			Self::Node(node) => node.halt_child(index),
			Self::Proxy(proxy) => proxy.halt_child(index),
		}
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		match self {
			Self::Leaf(leaf) => leaf.halt(index),
			Self::Node(node) => node.halt(index),
			Self::Proxy(proxy) => proxy.halt(index),
		}
	}
}
// endregion:	--- TreeElement

// region:      --- BehaviorTreeComponent
/// Interface for an element in a [`BehaviorTree`]
pub trait BehaviorTreeComponent: Send + Sync {
	/// Get the id
	fn id(&self) -> &str;

	/// Get the blackboard
	fn blackboard(&self) -> BlackboardNodeRef;

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
}
// endregion:   --- BehaviorTreeComponent
