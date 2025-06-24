// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! together with a [`proxy pattern`](https://en.wikipedia.org/wiki/Proxy_pattern)
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
#[cfg(feature = "std")]
use alloc::string::String;
use alloc::{sync::Arc, vec::Vec};
use dimas_scripting::SharedRuntime;
use libloading::Library;
use parking_lot::Mutex;

use crate::{
	behavior::{BehaviorError, BehaviorResult, BehaviorState},
	blackboard::SharedBlackboard,
	factory::BehaviorRegistry,
	tree::tree_iter::{TeeIterMut, TreeIter},
};

use super::{BehaviorTreeElement, error::Error};
// endregion:   --- modules

// region:		--- helper
/// Helper function to print a (sub)tree recursively
/// # Errors
/// - if recursion is deeper than 127
#[cfg(feature = "std")]
pub fn print_tree(start_node: &BehaviorTreeElement) -> Result<(), Error> {
	print_recursively(0, start_node)
}

/// Recursion function to print a (sub)tree recursively
/// # Errors
/// - Limit is a tree-depth of 127
#[cfg(feature = "std")]
fn print_recursively(level: i8, node: &BehaviorTreeElement) -> Result<(), Error> {
	if level == i8::MAX {
		return Err(Error::Unexpected(
			"recursion limit reached".into(),
			file!().into(),
			line!(),
		));
	}

	let next_level = level + 1;
	let mut indentation = String::new();
	for _ in 0..level {
		indentation.push_str("  ");
	}

	std::println!("{indentation}{}", node.data().description().name());
	for child in &**node.children() {
		print_recursively(next_level, child)?;
	}
	Ok(())
}
// endregion:	--- helper

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeElement`]s.
/// A certain [`BehaviorTree`] can contain up to 65536 [`BehaviorTreeElement`]s.
pub struct BehaviorTree {
	/// The root element
	root: BehaviorTreeElement,
	/// `runtime` is shared between elements
	runtime: SharedRuntime,
	/// `libraries` stores a reference to the used shared libraries aka plugins.
	/// This is necessary to avoid memory deallocation of libs while tree is in use.
	_libraries: Vec<Arc<Library>>,
}

impl BehaviorTree {
	/// create a Tree with reference to its libraries
	#[must_use]
	pub fn new(root: BehaviorTreeElement, registry: &BehaviorRegistry) -> Self {
		// create a clone of the scripting runtime
		let runtime = Arc::new(Mutex::new(registry.runtime().clone()));
		// clone the current state of registered libraries
		let mut libraries = Vec::new();
		for lib in registry.libraries() {
			libraries.push(lib.clone());
		}
		Self {
			root,
			runtime,
			_libraries: libraries,
		}
	}

	/// Access the root blackboard of the tree.
	#[must_use]
	pub const fn blackboard(&self) -> &SharedBlackboard {
		self.root.data().blackboard()
	}

	/// Access the root blackboard of the tree.
	#[must_use]
	pub const fn blackboard_mut(&mut self) -> &mut SharedBlackboard {
		self.root.data_mut().blackboard_mut()
	}

	/// Pretty print the tree.
	/// # Errors
	/// - if tree depth exceeds 127 (sub)tree levels.
	/// # Panics
	/// - if tree has no root
	pub fn print(&self) -> Result<(), Error> {
		print_recursively(0, &self.root)
	}

	/// Get a (sub)tree where index 0 is root tree.
	/// # Errors
	/// - if index is out of bounds.
	pub fn subtree(&self, _index: usize) -> Result<&BehaviorTreeElement, Error> {
		todo!("subtree access")
	}

	/// Ticks the tree exactly once.
	/// # Errors
	/// # Panics
	/// - if tree has no root
	pub async fn tick_exactly_once(&mut self) -> BehaviorResult {
		self.root.execute_tick(&self.runtime).await
	}

	/// Ticks the tree once.
	/// @TODO: The wakeup mechanism is not yet implemented
	/// # Errors
	/// # Panics
	/// - if tree has no root
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.root.execute_tick(&self.runtime).await
	}

	/// Ticks the tree until it finishes either with [`BehaviorState::Success`] or [`BehaviorState::Failure`].
	/// # Errors
	/// # Panics
	/// - if tree has no root
	///
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		let mut state = BehaviorState::Running;
		while state == BehaviorState::Running || state == BehaviorState::Idle {
			state = self.root.execute_tick(&self.runtime).await?;

			// Not implemented: Check for wake-up conditions and tick again if so
			// @TODO!

			// be cooperative & allow pending tasks to catch up
			// crucial for spawned tasks with bounded channels
			tokio::task::yield_now().await;
		}

		// halt eventually still running tasks
		self.root.execute_halt(&self.runtime).await?;

		// be cooperative & allow pending tasks to catch up
		// crucial for spawned tasks with bounded channels
		tokio::task::yield_now().await;

		Ok(state)
	}

	/// Get an iterator over the tree.
	pub fn iter(&self) -> impl Iterator<Item = &BehaviorTreeElement> {
		TreeIter::new(&self.root)
	}

	/// Get a mutable iterator over the tree.
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BehaviorTreeElement> {
		TeeIterMut::new(&mut self.root)
	}

	/// Reset tree to initial state.
	/// # Errors
	/// - if reset of children failed
	pub fn reset(&mut self) -> Result<(), BehaviorError> {
		self.root.reset(&self.runtime)?;
		self.runtime.lock().clear();
		Ok(())
	}
}
// endregion:	--- BehaviorTree
