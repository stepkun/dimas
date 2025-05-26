// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTreeElement`]
//!

// region:      --- modules
use crate::{
	behavior::{BehaviorPtr, BehaviorResult, BehaviorStatus, error::BehaviorError},
	blackboard::SharedBlackboard,
};
use dimas_core::BoxConstString;

use super::BehaviorTreeElementList;
// endregion:   --- modules

// region:		--- BehaviorTreeElement
/// A tree elements.
pub struct BehaviorTreeElement {
	/// UID of the element.
	/// 65536 [`BehaviorTreeElement`]s in a [`BehaviorTree`](crate::tree::BehaviorTree) should be sufficient.
	/// The ordering of the uid is following the creation order by the [`XmlParser`](crate::factory::xml_parser::XmlParser).
	/// This should end up in a depth first ordering.
	uid: i16,
	/// Name of the element.
	name: BoxConstString,
	/// Path to the element.
	/// In contrast to BehaviorTree.CPP this path is fully qualified, which means that every level is denoted explicitly.
	path: BoxConstString,
	/// Current [`BehaviorStatus`]
	status: BehaviorStatus,
	/// Reference to the [`Blackboard`] for the element.
	blackboard: SharedBlackboard,
	/// The behavior of that element.
	behavior: BehaviorPtr,
	/// Children of the element.
	children: BehaviorTreeElementList,
}

impl BehaviorTreeElement {
	/// Construct a [`BehaviorTreeElement`].
	///
	/// Non public to enforce using the dedicated creation functions.
	#[inline]
	fn new(
		uid: i16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self {
			uid,
			name: name.into(),
			path: path.into(),
			status: BehaviorStatus::Idle,
			blackboard,
			behavior,
			children,
		}
	}

	/// Create a tree leaf.
	#[must_use]
	pub fn create_leaf(
		uid: i16,
		name: &str,
		path: &str,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(
			uid,
			name,
			path,
			BehaviorTreeElementList::default(),
			blackboard,
			behavior,
		)
	}

	/// Create a tree node.
	#[must_use]
	pub fn create_node(
		uid: i16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(uid, name, path, children, blackboard, behavior)
	}

	/// Create a subtree.
	#[must_use]
	pub fn create_subtree(
		uid: i16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(uid, name, path, children, blackboard, behavior)
	}

	/// Get the uid.
	pub fn uid(&self) -> i16 {
		self.uid
	}

	/// Get the name.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Get the path.
	pub fn path(&self) -> &str {
		&self.path
	}

	/// Get a reference to the behavior.
	pub fn behavior(&self) -> &BehaviorPtr {
		&self.behavior
	}

	/// Get a mutable reference to the behavior.
	pub fn behavior_mut(&mut self) -> &mut BehaviorPtr {
		&mut self.behavior
	}

	/// Get the blackboard.
	pub fn blackboard(&self) -> SharedBlackboard {
		self.blackboard.clone()
	}

	/// Get the children.
	pub fn children(&self) -> &BehaviorTreeElementList {
		&self.children
	}

	/// Get the children mutable.
	pub fn children_mut(&mut self) -> &mut BehaviorTreeElementList {
		&mut self.children
	}

	/// Halt the element and all its children.
	/// # Errors
	pub fn execute_halt(&mut self) -> Result<(), BehaviorError> {
		self.halt(0)
	}

	/// Tick the element and its children.
	/// # Errors
	pub fn execute_tick(&mut self) -> BehaviorResult {
		let status = if self.status == BehaviorStatus::Idle {
			self.behavior
				.start(self.status, &mut self.blackboard, &mut self.children)?
		} else {
			self.behavior
				.tick(self.status, &mut self.blackboard, &mut self.children)?
		};
		self.status = status;
		Ok(status)
	}

	/// Halt child at `index`.
	/// # Errors
	/// - if index is out of childrens bounds.
	pub fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt_child(index)
	}

	/// Halt all children at and beyond `index`.
	/// # Errors
	/// - if index is out of childrens bounds.
	pub fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt(index)
	}

	/// Return an iterator over the children.
	#[must_use]
	pub fn children_iter(&self) -> impl DoubleEndedIterator<Item = &Self> {
		self.children().iter()
	}

	/// Return a mutable iterator over the children.
	#[must_use]
	pub fn children_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
		self.children_mut().iter_mut()
	}
}
// endregion:	--- BehaviorTreeElement
