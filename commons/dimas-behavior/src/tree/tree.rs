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
pub fn print_tree(start_node: &dyn BehaviorTreeComponent) {
	std::println!("{}", start_node.id());
	print_recursively(0, start_node);
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
	#[allow(clippy::needless_pass_by_value)]
	fn link_subtree(&self, subtree: BehaviorSubTree) -> Result<(), Error> {
		let mut node = &mut *subtree.lock();
		for mut child in &mut node.children_mut().0 {
			self.recursive_node(child.as_mut())?;
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	#[allow(clippy::needless_pass_by_value)]
	#[allow(clippy::unused_self)]
	#[allow(clippy::match_bool)]
	#[allow(clippy::single_match_else)]
	#[allow(unsafe_code)]
	fn recursive_node(&self, node: &mut dyn BehaviorTreeComponent) -> Result<(), Error> {
		if let Some(proxy) = node
			.as_any_mut()
			.downcast_mut::<BehaviorTreeProxy>()
		{
			let id = proxy.id();
			let subtree = self.subtree_by_name(id)?;
			proxy.set_subtree(subtree);
		} else if let Some(node) = node
			.as_any_mut()
			.downcast_mut::<BehaviorTreeNode>()
		{
			for mut child in &mut node.children_mut().0 {
				self.recursive_node(child.as_mut())?;
			}
		}
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
			let guard = node.lock();
			std::println!("{}", guard.id());
			print_recursively(0, &*guard)
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

	/// Find a subtree in the list and return a reference to it
	///
	fn subtree_by_name(&self, id: &str) -> Result<BehaviorSubTree, Error> {
		for subtree in &self.subtrees {
			// if subtree contains himself, this will become a deadlock
			if let Some(intern) = subtree.try_lock() {
				if intern.id() == id {
					return Ok(subtree.clone());
				}
			} else {
				return Err(Error::DeadLock(id.into()));
			}
		}
		Err(Error::SubtreeNotFound(id.into()))
	}
}
// endregion:	--- BehaviorTree

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

	pub(crate) fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.0[index].halt(0)
	}

	pub(crate) fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.0[index].halt_child(0)
	}
}
// endregion:	--- BehaviorTreeComponentList
