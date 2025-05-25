// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTree`] library
//!

pub mod error;
#[allow(clippy::module_inception)]
mod tree;
mod tree_element;
mod tree_element_list;

// flatten
pub use tree::{BehaviorTree, print_tree};
pub use tree_element::BehaviorTreeElement;
pub use tree_element_list::BehaviorTreeElementList;

// region:      --- modules
use crate::{
	behavior::{BehaviorPtr, BehaviorResult, error::BehaviorError},
	blackboard::SharedBlackboard,
};
// endregion:   --- modules

// region:      --- BehaviorTreeComponent
/// Interface for an element in a [`BehaviorTree`]
pub trait BehaviorTreeComponent: Send + Sync {
	/// Get the id
	fn id(&self) -> &str;

	/// Get the name
	fn name(&self) -> &str;

	/// Get the path
	fn path(&self) -> &str;

	/// Get the behavior
	fn behavior(&self) -> &BehaviorPtr;

	/// Get the behavior
	fn behavior_mut(&mut self) -> &mut BehaviorPtr;

	/// Get the blackboard
	fn blackboard(&self) -> SharedBlackboard;

	/// Get the children
	fn children(&self) -> &BehaviorTreeElementList;

	/// Get the children mutable
	fn children_mut(&mut self) -> &mut BehaviorTreeElementList;

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
