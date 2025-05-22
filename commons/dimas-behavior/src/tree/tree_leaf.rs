// Copyright © 2025 Stephan Kunz

//! [`BehaviorTreeLeaf`] implementation.
//!

// region:      --- modules
use dimas_core::BoxConstString;

use crate::{
	behavior::{
		BehaviorPtr, BehaviorResult, BehaviorStatus, BehaviorTickData, error::BehaviorError,
	},
	blackboard::SharedBlackboard,
};

use super::{BehaviorTreeComponent, BehaviorTreeComponentList, BehaviorTreeElement};
// endregion:   --- modules

// region:		--- BehaviorTreeLeaf
/// Implementation of a trees leaf
pub struct BehaviorTreeLeaf {
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
	/// Dummy children list.
	children: BehaviorTreeComponentList,
}

impl BehaviorTreeComponent for BehaviorTreeLeaf {
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

	fn children(&self) -> &BehaviorTreeComponentList {
		&self.children
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList {
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

	fn halt_child(&mut self, _index: usize) -> Result<(), BehaviorError> {
		Ok(())
	}

	fn halt(&mut self, _index: usize) -> Result<(), BehaviorError> {
		Ok(())
	}
}

impl BehaviorTreeLeaf {
	/// Construct a [`BehaviorTreeLeaf`]
	#[must_use]
	pub fn new(
		id: &str,
		path: &str,
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
			children: BehaviorTreeComponentList::default(),
		}
	}

	/// Create a tree leaf <code>TreeElement::Leaf(BehaviorTreeLeaf)</code>.
	#[must_use]
	pub fn create(
		id: &str,
		path: &str,
		tick_data: BehaviorTickData,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
	) -> BehaviorTreeElement {
		BehaviorTreeElement::Leaf(Self::new(id, path, tick_data, blackboard, behavior))
	}
}
// endregion:	--- BehaviorTreeLeaf
