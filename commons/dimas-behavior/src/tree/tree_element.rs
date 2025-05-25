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
	/// ID of the node.
	id: BoxConstString,
	/// Path to the node.
	path: BoxConstString,
	/// Data needed in every tick.
	tick_data: BehaviorTickData,
	/// Reference to the [`Blackboard`] for the leaf.
	blackboard: SharedBlackboard,
	/// The behavior of that leaf.
	behavior: BehaviorPtr,
	/// Children.
	children: BehaviorTreeElementList,
}

impl BehaviorTreeComponent for BehaviorTreeElement {
	fn id(&self) -> &str {
		&self.id
	}

	fn name(&self) -> &str {
		&self.id
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
		id: &str,
		path: &str,
		children: BehaviorTreeElementList,
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self {
			id: id.into(),
			path: path.into(),
			tick_data,
			blackboard,
			behavior,
			children,
		}
	}

	/// Create a tree node.
	#[must_use]
	pub fn create_node(
		id: &str,
		path: &str,
		children: BehaviorTreeElementList,
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(id, path, children, tick_data, blackboard, behavior)
	}

	/// Create a tree leaf.
	#[must_use]
	pub fn create_leaf(
		id: &str,
		path: &str,
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> Self {
		Self::new(
			id,
			path,
			BehaviorTreeElementList::default(),
			tick_data,
			blackboard,
			behavior,
		)
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
