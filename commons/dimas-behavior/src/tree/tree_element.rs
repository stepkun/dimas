// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTreeElement`]
//!

// region:      --- modules
use crate::{
	behavior::{
		BehaviorPtr, BehaviorResult, BehaviorStatus, BehaviorTickData, error::BehaviorError,
	},
	blackboard::SharedBlackboard,
};
use dimas_core::BoxConstString;

use super::{BehaviorTreeComponent, BehaviorTreeElementList};
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
	/// Data needed in every tick.
	tick_data: BehaviorTickData,
	/// Reference to the [`Blackboard`] for the element.
	blackboard: SharedBlackboard,
	/// The behavior of that element.
	behavior: BehaviorPtr,
	/// Children of the element.
	children: BehaviorTreeElementList,
}

impl BehaviorTreeComponent for BehaviorTreeElement {
	fn uid(&self) -> i16 {
		self.uid
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn path(&self) -> &str {
		&self.path
	}

	fn behavior(&self) -> &BehaviorPtr {
		&self.behavior
	}

	fn behavior_mut(&mut self) -> &mut BehaviorPtr {
		&mut self.behavior
	}

	fn blackboard(&self) -> SharedBlackboard {
		self.blackboard.clone()
	}

	fn children(&self) -> &BehaviorTreeElementList {
		&self.children
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeElementList {
		&mut self.children
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status();
		if status == BehaviorStatus::Idle {
			status = self.behavior.start(
				&mut self.tick_data,
				&mut self.blackboard,
				&mut self.children,
			)?;
		} else {
			status = self.behavior.tick(
				&mut self.tick_data,
				&mut self.blackboard,
				&mut self.children,
			)?;
		}
		self.tick_data.set_status(status);
		Ok(status)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt_child(index)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt(index)
	}
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
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self {
			uid,
			name: name.into(),
			path: path.into(),
			tick_data,
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
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(
			uid,
			name,
			path,
			BehaviorTreeElementList::default(),
			tick_data,
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
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(uid, name, path, children, tick_data, blackboard, behavior)
	}

	/// Create a subtree.
	#[must_use]
	pub fn create_subtree(
		uid: i16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(uid, name, path, children, tick_data, blackboard, behavior)
	}

	/// Return an iterator over the children
	#[must_use]
	pub fn children_iter(&self) -> impl DoubleEndedIterator<Item = &Self> {
		self.children().iter()
	}

	/// Return a mutable iterator over the children
	#[must_use]
	pub fn children_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
		self.children_mut().iter_mut()
	}
}
// endregion:	--- BehaviorTreeElement
