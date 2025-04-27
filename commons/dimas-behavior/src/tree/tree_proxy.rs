// Copyright © 2025 Stephan Kunz
// #![allow(clippy::unused_async)]
// #![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! together with a [`proxy pattern`](https://en.wikipedia.org/wiki/Proxy_pattern)
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::format;
use dimas_core::ConstString;

use crate::{
	behavior::{BehaviorResult, BehaviorTickData, error::BehaviorError},
	blackboard::Blackboard,
};

use super::{BehaviorSubTree, BehaviorTreeComponent, BehaviorTreeComponentList, TreeElement};
// endregion:   --- modules

// region:		--- BehaviorTreeProxy
/// Implementation of a trees proxy node
pub struct BehaviorTreeProxy {
	/// ID of the node
	id: ConstString,
	/// The Subtree to call
	subtree: Option<BehaviorSubTree>,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// empty dummy list
	children: BehaviorTreeComponentList,
}

impl BehaviorTreeComponent for BehaviorTreeProxy {
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
		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.write().execute_tick(),
		)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > 0 {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.write().execute_halt(),
		)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > 0 {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.write().execute_halt(),
		)
	}
}

impl BehaviorTreeProxy {
	/// Construct a [`BehaviorTreeProxy`]
	#[must_use]
	pub fn new(id: &str, tick_data: BehaviorTickData) -> Self {
		Self {
			id: id.into(),
			subtree: None,
			tick_data,
			children: BehaviorTreeComponentList::default(),
		}
	}

	/// Create a [`TreeElement`]`::Proxy`([`BehaviorTreeProxy`])
	#[must_use]
	pub fn create(id: &str, tick_data: BehaviorTickData) -> TreeElement {
		TreeElement::Proxy(Self::new(id, tick_data))
	}

	pub(crate) fn set_subtree(&mut self, subtree: BehaviorSubTree) {
		self.subtree = Some(subtree);
	}

	#[allow(clippy::redundant_closure_for_method_calls)]
	pub(crate) fn subtree(&self) -> Option<BehaviorSubTree> {
		self.subtree.as_ref().map(|sub| sub.clone())
	}
}
// endregion:	--- BehaviorTreeProxy
