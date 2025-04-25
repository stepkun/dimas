// Copyright © 2025 Stephan Kunz
// #![allow(clippy::unused_async)]
// #![allow(unused)]

//! [`BehaviorTreeLeaf`] implementation.
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use dimas_core::ConstString;

use crate::{
	behavior::{
		BehaviorPtr, BehaviorResult, BehaviorStatus, BehaviorTickData, error::BehaviorError,
	},
	blackboard::Blackboard,
};

use super::{BehaviorTreeComponent, BehaviorTreeComponentList, TreeElement};
// endregion:   --- modules

// region:		--- BehaviorTreeLeaf
/// Implementation of a trees leaf
pub struct BehaviorTreeLeaf {
	/// ID of the node
	id: ConstString,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// The behavior of that leaf
	behavior: BehaviorPtr,
	/// dummy children list
	children: BehaviorTreeComponentList,
}

impl BehaviorTreeComponent for BehaviorTreeLeaf {
	fn id(&self) -> &str {
		&self.id
	}

	fn blackboard(&self) -> Blackboard {
		self.tick_data.blackboard.clone()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		&self.children
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList {
		&mut self.children
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status;
		if status == BehaviorStatus::Idle {
			status = self
				.behavior
				.start(&mut self.tick_data, &mut self.children)?;
		} else {
			status = self
				.behavior
				.tick(&mut self.tick_data, &mut self.children)?;
		}
		self.tick_data.status = status;
		Ok(status)
	}

	fn halt_child(&mut self, _index: usize) -> Result<(), BehaviorError> {
		self.behavior.halt(&mut self.children)
	}

	fn halt(&mut self, _index: usize) -> Result<(), BehaviorError> {
		self.behavior.halt(&mut self.children)
	}
}

impl BehaviorTreeLeaf {
	/// Construct a [`BehaviorTreeNode`]
	#[must_use]
	pub fn new(id: &str, tick_data: BehaviorTickData, behavior: BehaviorPtr) -> Self {
		Self {
			id: id.into(),
			tick_data,
			behavior,
			children: BehaviorTreeComponentList::default(),
		}
	}

	/// Create a [`TreeElement`]`::Leaf`([`BehaviorTreeLeaf`])
	#[must_use]
	pub fn create(
		id: &str,
		tick_data: BehaviorTickData,
		behavior: BehaviorPtr,
	) -> TreeElement {
		TreeElement::Leaf(Self::new(id, tick_data, behavior))
	}
}
// endregion:	--- BehaviorTreeLeaf
