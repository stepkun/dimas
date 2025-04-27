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
use libloading::Library;
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
		error::BehaviorError, BehaviorConfigurationData, BehaviorInstanceMethods, BehaviorResult, BehaviorStatus, BehaviorTickData, BehaviorTreeMethods
	},
	blackboard::Blackboard, factory::BehaviorRegistry,
};

use super::{
	error::Error, BehaviorSubTree, BehaviorTreeComponent, BehaviorTreeLeaf, BehaviorTreeNode, BehaviorTreeProxy, TreeElement
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
		print_recursively(next_level, child);
	}
	Ok(())
}

// endregion:	--- helper

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeComponent`]s
pub struct BehaviorTree {
	pub(crate) root: BehaviorSubTree,
	pub(crate) subtrees: Vec<BehaviorSubTree>,
	pub(crate) libraries: Vec<Arc<Library>>,
}

impl BehaviorTree {
	/// create a Tree with reference to its libraries
	pub fn new(root: BehaviorSubTree, registry: &BehaviorRegistry) -> Self {
		let subtrees = Vec::new();
		// @TODO: create a list of all used subtrees
		// clone the current state of registered libraries
		let mut libraries = Vec::new();
		for lib in registry.libraries() {
			libraries.push(lib.clone());
		};
		Self {
			root,
			subtrees,
			libraries,
		}
	}

	/// Pretty print the tree
	/// # Errors
	/// - if root tree is not yet set
	#[allow(clippy::option_if_let_else)]
	pub fn print(&self) -> Result<(), Error> {
		let guard = self.root.lock();
		std::println!("{}", guard.id());
		print_recursively(0, &*guard)
	}

	/// Get a (sub)tree where index 0 is root tree
	/// # Errors
	/// - if no root tree is set
	/// - if index is out of bounds
	pub fn subtree(&self, index: usize) -> Result<BehaviorSubTree, Error> {
		if index == 0 {
			Ok(self.root.clone())
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

		while status == BehaviorStatus::Idle || matches!(status, BehaviorStatus::Running) {
			status = self.root.lock().execute_tick()?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				self.root.lock().halt(0)?;
				break;
			}
		}
		Ok(status)
	}

	/// Ticks the tree exactly once
	/// # Errors
	/// - if no root exists
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.root.lock().execute_tick()
	}

	/// Find a subtree in the list and return a reference to it
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
pub struct BehaviorTreeComponentList(pub(crate) Vec<TreeElement>);

impl Deref for BehaviorTreeComponentList {
	type Target = Vec<TreeElement>;

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
