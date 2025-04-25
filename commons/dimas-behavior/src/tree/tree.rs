// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! together with a [`proxy pattern`](https://en.wikipedia.org/wiki/Proxy_pattern)
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec,
	vec::Vec,
};
use core::{
	any::{Any, TypeId},
	marker::PhantomData,
	ops::{Deref, DerefMut},
};
use dimas_core::ConstString;
use dimas_scripting::{Parser, VM};
use hashbrown::HashMap;
use parking_lot::Mutex;
use rustc_hash::FxBuildHasher;

use crate::{
	behavior::{
		BehaviorConfigurationData, BehaviorInstanceMethods, BehaviorResult, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, error::BehaviorError,
	},
	blackboard::Blackboard,
};

use super::{
	BehaviorSubTree, BehaviorTreeComponent, BehaviorTreeLeaf, BehaviorTreeNode, BehaviorTreeProxy,
	error::Error,
};
// endregion:   --- modules

// region:		--- helper
/// Helper function to print a (sub)tree recursively
#[cfg(feature = "std")]
pub fn print_tree(root_node: &dyn BehaviorTreeComponent) {
	std::println!("{}", root_node.id());
	print_recursively(0, root_node);
}

/// Helper function to print a (sub)tree recursively
/// Recursion function to print a (sub)tree recursively
/// # Errors
/// - Limit is a tree-depth of 127
#[cfg(feature = "std")]
#[allow(clippy::needless_pass_by_value)]
fn print_recursively(level: i8, node: &dyn BehaviorTreeComponent) -> Result<(), Error> {
	if level == i8::MAX {
		return Err(Error::Unexpected(
			"recursion limit reached".into(),
			file!().into(),
			line!(),
		));
	}

	let next_level = level + 1;
	let mut indentation = String::new();
	for _ in 0..next_level {
		indentation.push_str("   |");
	}
	for child in &**node.children() {
		std::println!("{}- {}", indentation, child.id());
		print_recursively(next_level, child.as_ref());
	}
	Ok(())
}

// endregion:	--- helper

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeComponent`]s
#[derive(Default)]
pub struct BehaviorTree {
	pub(crate) root: Option<BehaviorSubTree>,
	pub(crate) subtrees: Vec<BehaviorSubTree>,
}

impl BehaviorTree {
	/// Set the root of the tree
	pub(crate) fn set_root(&mut self, root: BehaviorTreeNode) {
		self.root = Some(Arc::new(Mutex::new(root)));
	}

	/// Add a subtree
	pub(crate) fn add_subtree(&mut self, subtree: BehaviorTreeNode) {
		self.subtrees.push(Arc::new(Mutex::new(subtree)));
	}

	/// Link each Proxy in a subtree to its subtree
	#[allow(clippy::unnecessary_wraps)]
	#[allow(clippy::needless_pass_by_value)]
	#[allow(clippy::unused_self)]
	#[allow(clippy::match_bool)]
	#[allow(clippy::single_match_else)]
	#[allow(unsafe_code)]
	fn link_subtree(&self, subtree: BehaviorSubTree) -> Result<(), Error> {
		let mut node = &mut *subtree.lock();

		std::dbg!(node.id());

		// let mut iter = subtree.lock();
		// for mut child in iter.by_ref().next().expect("snh") {
		// 	match child.deref().type_id() == TypeId::of::<BehaviorTreeProxy>() {
		// 		true => {
		// 			let component = &**child;

		// 			return Err(Error::SubtreeNotFound("ToDo".into()));
		// 		}
		// 		false => {
		// 			// ignore
		// 		}
		// 	};
		// };
		//drop(iter);

		Ok(())
	}

	/// Link each Proxy in the tree to its subtree
	pub(crate) fn link_tree(&self) -> Result<(), Error> {
		if let Some(root) = self.root.clone() {
			self.link_subtree(root)?;
		} else {
			return Err(Error::RootNotFound("Root".into()));
		}

		for subtree in self.subtrees.clone() {
			self.link_subtree(subtree)?;
		}

		Ok(())
	}

	/// Pretty print the tree
	/// # Errors
	/// - if root tree is not yet set
	#[allow(clippy::option_if_let_else)]
	pub fn print(&self) -> Result<(), Error> {
		if let Some(node) = &self.root {
			std::println!("{}", node.lock().id());
			print_recursively(0, node.lock().by_ref())
		} else {
			Err(Error::RootNotFound("TODO!".into()))
		}
	}

	/// Get a (sub)tree where index 0 is root tree
	/// # Errors
	/// - if no root tree is set
	/// - if index is out of bounds
	pub fn subtree(&self, index: usize) -> Result<BehaviorSubTree, Error> {
		if index == 0 {
			let res = self
				.root
				.as_ref()
				.ok_or(Error::IndexOutOfBounds(0))?;
			Ok(res.clone())
		} else if (index - 1) > self.subtrees.len() {
			Err(Error::IndexOutOfBounds(index))
		} else {
			Ok(self.subtrees[index - 1].clone())
		}
	}

	/// Ticks the tree until it finishes either with [`BehaviorStatus::Success`] or [`BehaviorStatus::Failure`]
	/// # Errors
	/// - if no root exists
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		let mut status = BehaviorStatus::Idle;

		if let Some(root) = &mut self.root {
			while status == BehaviorStatus::Idle || matches!(status, BehaviorStatus::Running) {
				status = root.lock().execute_tick()?;

				// Not implemented: Check for wake-up conditions and tick again if so

				if status.is_completed() {
					root.lock().halt(0)?;
					break;
				}
			}
			Ok(status)
		} else {
			Err(BehaviorError::RootNotFound("@TODO: 1".into()))
		}
	}

	/// Ticks the tree exactly once
	/// # Errors
	/// - if no root exists
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.root.as_ref().map_or_else(
			|| Err(BehaviorError::RootNotFound("@TODO: 2".into())),
			|root| root.lock().execute_tick(),
		)
	}
}
// endregion:	--- BehaviorTree

// region: 		--- TreeComponentIter
/// Iterator over the [`BehaviorTreeComponentPtr`]
/// @TODO:
#[allow(clippy::borrowed_box)]
pub struct TreeComponentIter<'a> {
	/// stack to do a depth first search
	stack: Vec<&'a Box<dyn BehaviorTreeComponent>>,
	/// Lifetime marker
	marker: PhantomData<Box<dyn BehaviorTreeComponent>>,
}

#[allow(clippy::borrowed_box)]
impl<'a> TreeComponentIter<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a Box<dyn BehaviorTreeComponent>) -> Self {
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

#[allow(clippy::borrowed_box)]
impl<'a> Iterator for TreeComponentIter<'a> {
	type Item = &'a Box<dyn BehaviorTreeComponent>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(component) = self.stack.pop() {
			if component.deref().type_id() != TypeId::of::<BehaviorTreeLeaf>() {
				// Push children in revers order to maintain left-to-right order
				let list = component.children().deref().iter().rev();
				for child in list {
					self.stack.push(child);
				}
			}
			return Some(component);
		}
		None
	}
}
// endregion:	--- TreeComponentIter

// region: 		--- TreeComponentIterMut
/// Mutable Iterator over the [`BehaviorTree`]
/// @TODO:
pub struct TreeComponentIterMut<'a> {
	/// stack to do a depth first search
	stack: Vec<*mut Box<dyn BehaviorTreeComponent>>,
	/// Lifetime marker
	marker: PhantomData<&'a mut Box<dyn BehaviorTreeComponent>>,
}

#[allow(clippy::needless_lifetimes)]
impl<'a> TreeComponentIterMut<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: *mut Box<dyn BehaviorTreeComponent>) -> Self {
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

#[allow(clippy::needless_lifetimes)]
#[allow(unsafe_code)]
impl<'a> Iterator for TreeComponentIterMut<'a> {
	type Item = *mut Box<dyn BehaviorTreeComponent>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(component_ptr) = self.stack.pop() {
			// we know this pointer is valid since the iterator owns the traversal
			let component = unsafe { &mut *component_ptr };
			if component.deref().deref().type_id() != TypeId::of::<BehaviorTreeLeaf>() {
				// Push children in revers order to maintain left-to-right order
				let iter = component
					.children_mut()
					.deref_mut()
					.iter_mut()
					.rev();
				for child in iter.rev() {
					self.stack.push(child);
				}
			}
			return Some(&mut *component);
		}
		None
	}
}
// endregion:	--- TreeComponentIterMut

// region:		--- BehaviorTreeComponentList
/// A List of tree components
#[derive(Default)]
pub struct BehaviorTreeComponentList(Vec<Box<dyn BehaviorTreeComponent>>);

impl Deref for BehaviorTreeComponentList {
	type Target = Vec<Box<dyn BehaviorTreeComponent>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BehaviorTreeComponentList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl BehaviorTreeComponent for BehaviorTreeComponentList {
	fn id(&self) -> &'static str {
		"BehaviorTreeComponentList"
	}

	fn blackboard(&self) -> Blackboard {
		Blackboard::default()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		self
	}

	fn children_mut(&mut self) -> &mut BehaviorTreeComponentList {
		&mut *self
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		for item in &mut self.0 {
			item.execute_tick()?;
		}
		Ok(BehaviorStatus::Success)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > self.0.len() {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.0[index].execute_halt()
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > self.0.len() {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		for child in &mut *self.0 {
			child.execute_halt()?;
		}
		Ok(())
	}
}

impl Iterator for BehaviorTreeComponentList {
	type Item = TreeComponentIter<'static>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[allow(clippy::needless_lifetimes)]
impl<'a> BehaviorTreeComponentList {
	/// Reset all children
	/// # Errors
	pub fn reset(&'a mut self) -> Result<(), BehaviorError> {
		let x = &mut self.0;
		for child in x {
			child.halt(0)?;
		}
		Ok(())
	}
}
// endregion:	--- BehaviorTreeComponentList
