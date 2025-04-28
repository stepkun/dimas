// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! together with a [`proxy pattern`](https://en.wikipedia.org/wiki/Proxy_pattern)
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{string::String, sync::Arc, vec::Vec};
use core::ops::{Deref, DerefMut};
use libloading::Library;

use crate::{
	behavior::{BehaviorResult, BehaviorStatus, error::BehaviorError},
	factory::BehaviorRegistry,
};

use super::{BehaviorSubTree, BehaviorTreeComponent, TreeElement, error::Error};
// endregion:   --- modules

// region:		--- helper
/// Helper function to print a (sub)tree recursively
/// # Errors
/// - if recursion is deeper than 127
#[cfg(feature = "std")]
pub fn print_tree(start_node: &TreeElement) -> Result<(), Error> {
	std::println!("{}", start_node.id());
	print_recursively(0, start_node)
}

/// Helper function to print a (sub)tree recursively
/// Recursion function to print a (sub)tree recursively
/// # Errors
/// - Limit is a tree-depth of 127
#[cfg(feature = "std")]
fn print_recursively(level: i8, node: &TreeElement) -> Result<(), Error> {
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
	match node {
		TreeElement::Leaf(leaf) => {
			std::println!("{indentation}- {}", leaf.id());
		}
		TreeElement::Node(node) => {
			std::println!("{indentation}- {}", node.id());
			for child in &**node.children() {
				print_recursively(next_level, child)?;
			}
		}
		TreeElement::Proxy(proxy) => {
			std::println!("{indentation}- SubTree: {}", proxy.id());
			if let Some(subtree) = proxy.subtree() {
				for child in &**subtree.read().children() {
					print_recursively(next_level, child)?;
				}
			} else {
				std::println!("{indentation}   |- missing!!");
			}
		}
	}
	Ok(())
}
// endregion:	--- helper

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeComponent`]s
pub struct BehaviorTree {
	pub(crate) root: BehaviorSubTree,
	pub(crate) subtrees: Vec<BehaviorSubTree>,
	pub(crate) _libraries: Vec<Arc<Library>>,
}

impl BehaviorTree {
	/// create a Tree with reference to its libraries
	pub fn new(root: BehaviorSubTree, registry: &BehaviorRegistry) -> Self {
		// @TODO: create a list of all used subtrees
		// for now its just a stupid copy of what is registered
		let mut subtrees = Vec::new();
		for sub in registry.subtrees() {
			subtrees.push(sub.clone());
		}

		// clone the current state of registered libraries
		let mut libraries = Vec::new();
		for lib in registry.libraries() {
			libraries.push(lib.clone());
		}
		Self {
			root,
			subtrees,
			_libraries: libraries,
		}
	}

	/// Pretty print the tree
	/// # Errors
	/// - if root tree is not yet set
	#[allow(clippy::option_if_let_else)]
	pub fn print(&self) -> Result<(), Error> {
		std::println!("{}", self.root.read().id());
		print_recursively(0, &self.root.read())
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
			status = self.root.write().execute_tick()?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				self.root.write().halt(0)?;
				break;
			}
		}
		Ok(status)
	}

	/// Ticks the tree exactly once
	/// # Errors
	/// - if no root exists
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.root.write().execute_tick()
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
