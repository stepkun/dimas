// Copyright © 2025 Stephan Kunz

//! [`BehaviorTree`] library
//!

pub mod error;
#[allow(clippy::module_inception)]
mod tree;

// flatten
pub use tree::{
	BehaviorTree, BehaviorTreeComponentList, BehaviorTreeLeaf, BehaviorTreeNode, BehaviorTreeProxy,
};

// region:      --- modules
use alloc::sync::Arc;
use parking_lot::Mutex;

use crate::behavior::{BehaviorResult, error::BehaviorError};
// endregion:   --- modules

//  region:		--- types
/// Shorthand for a behavior subtree definition
/// An `Arc` with `Mutex` to enable reusability in the tree.
pub type BehaviorSubTree = Arc<Mutex<BehaviorTreeNode>>;

// endregion:	--- types

// region:      --- BehaviorTreeComponent
/// Interface for an element in a [`BehaviorTree`]
pub trait BehaviorTreeComponent: Send + Sync {
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
