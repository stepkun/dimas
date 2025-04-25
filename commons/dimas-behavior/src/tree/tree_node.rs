// Copyright © 2025 Stephan Kunz
// #![allow(clippy::unused_async)]
// #![allow(unused)]

//! [`BehaviorTreeNode`] implementation.
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{boxed::Box, vec, vec::Vec};
use core::{
	any::{Any, TypeId},
	marker::PhantomData,
	ops::DerefMut,
};
use dimas_core::ConstString;

use crate::{
	behavior::{
		BehaviorPtr, BehaviorResult, BehaviorStatus, BehaviorTickData, error::BehaviorError,
	},
	blackboard::Blackboard,
};

use super::{BehaviorTreeComponent, BehaviorTreeComponentList, BehaviorTreeLeaf};
// endregion:   --- modules

// region:		--- BehaviorTreeNode
/// Implementation of a trees node
pub struct BehaviorTreeNode {
	/// ID of the node
	id: ConstString,
	/// Children
	children: BehaviorTreeComponentList,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// The behavior of that leaf
	behavior: BehaviorPtr,
}

impl BehaviorTreeComponent for BehaviorTreeNode {
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

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt_child(index)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		//self.behavior.halt(&mut self.children)
		self.children.halt(index)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl BehaviorTreeNode {
	/// Construct a [`BehaviorTreeNode`]
	#[must_use]
	pub fn new(
		id: &str,
		children: BehaviorTreeComponentList,
		tick_data: BehaviorTickData,
		behavior: BehaviorPtr,
	) -> Self {
		Self {
			id: id.into(),
			children,
			tick_data,
			behavior,
		}
	}

	/// Create a [`BehaviorTreeComponentPtr`]
	#[must_use]
	pub fn create(
		id: &str,
		children: BehaviorTreeComponentList,
		tick_data: BehaviorTickData,
		behavior: BehaviorPtr,
	) -> Box<dyn BehaviorTreeComponent> {
		Box::new(Self::new(id, children, tick_data, behavior))
	}

	/// Get the id
	#[must_use]
	pub const fn id(&self) -> &str {
		&self.id
	}

	/// Provide an Iterator
	pub fn iter(&self) -> impl Iterator<Item = &dyn BehaviorTreeComponent> {
		TreeNodeIter::new(self)
	}

	/// Provide a mutable Iterator
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn BehaviorTreeComponent> {
		TreeNodeIterMut::new(self)
	}
}
// endregion:	--- BehaviorTreeNode

// region: 		--- TreeNodetIter
/// Iterator over the [`BehaviorTreeNode`]
/// @TODO:
#[allow(dead_code)]
pub struct TreeNodeIter<'a> {
	/// stack to do a depth first search
	stack: Vec<&'a dyn BehaviorTreeComponent>,
	/// Lifetime marker
	marker: PhantomData<&'a dyn BehaviorTreeComponent>,
}

#[allow(clippy::needless_lifetimes)]
#[allow(unsafe_code)]
impl<'a> TreeNodeIter<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a dyn BehaviorTreeComponent) -> Self {
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

#[allow(clippy::needless_lifetimes)]
impl<'a> Iterator for TreeNodeIter<'a> {
	type Item = &'a dyn BehaviorTreeComponent;

	fn next(&mut self) -> Option<Self::Item> {
		// if let Some(component) = self.stack.pop() {
		// 	//let component = unsafe { & *component };
		// 	if component.type_id() != TypeId::of::<BehaviorTreeLeaf>() {
		// 		// Push children in reverse order to maintain left-to-right order
		// 		let iter = component
		// 			.children()
		// 			.iter();
		// 		for child in iter.rev() {
		// 			let child = &**child;
		// 			self.stack.push(child);
		// 		}
		// 	};
		// 	return Some(component);
		// };
		None
	}
}
// endregion:	--- TreeNodetIter

// region: 		--- TreeNodetIterMut
/// Mutable Iterator over the [`BehaviorTreeNode`]
/// @TODO:
pub struct TreeNodeIterMut<'a> {
	/// stack to do a depth first search
	stack: Vec<*mut dyn BehaviorTreeComponent>,
	/// Lifetime marker
	marker: PhantomData<&'a mut dyn BehaviorTreeComponent>,
}

#[allow(clippy::needless_lifetimes)]
#[allow(unsafe_code)]
impl<'a> TreeNodeIterMut<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a mut BehaviorTreeNode) -> Self {
		let root: &mut dyn BehaviorTreeComponent = root;
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

#[allow(clippy::needless_lifetimes)]
#[allow(unsafe_code)]
impl<'a> Iterator for TreeNodeIterMut<'a> {
	type Item = &'a mut dyn BehaviorTreeComponent;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(component_ptr) = self.stack.pop() {
			// we know this pointer is valid since the iterator owns the traversal
			let component = unsafe { &mut *component_ptr };
			if component.type_id() != TypeId::of::<BehaviorTreeLeaf>() {
				// Push children in reverse order to maintain left-to-right order
				let iter = component.children_mut().deref_mut().iter_mut();
				for child in iter.rev() {
					self.stack.push(&mut (**child));
				}
			};
			return Some(component);
		}
		None
	}
}
// endregion:	--- TreeNodetIterMut
