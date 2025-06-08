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
use alloc::{sync::Arc, vec, vec::Vec};
#[cfg(feature = "std")]
use alloc::string::String;
#[cfg(feature = "spawn")]
use alloc::string::ToString;
use core::marker::PhantomData;
use dimas_scripting::SharedRuntime;
use libloading::Library;
use parking_lot::Mutex;

use crate::{
	behavior::{BehaviorResult, BehaviorState},
	factory::BehaviorRegistry,
};
#[cfg(feature = "spawn")]
use crate::behavior::BehaviorError;

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

	std::println!("{indentation}{}", node.name());
	for child in &**node.children() {
		print_recursively(next_level, child)?;
	}
	Ok(())
}
// endregion:	--- helper

// region:		--- TreeIter
/// Iterator over the [`BehaviorTree`]
struct TreeIter<'a> {
	/// stack to do a depth first search
	stack: Vec<&'a BehaviorTreeElement>,
	/// Lifetime marker
	marker: PhantomData<&'a BehaviorTreeElement>,
}

impl<'a> TreeIter<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: Option<&'a BehaviorTreeElement>) -> Self {
		Self {
			stack: vec![root.as_ref().expect("snh")],
			marker: PhantomData,
		}
	}
}

impl<'a> Iterator for TreeIter<'a> {
	type Item = &'a BehaviorTreeElement;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::cast_possible_wrap)]
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(node) = self.stack.pop() {
			// Push children in revers order to maintain left-to-right order
			for child in node.children_iter().rev() {
				self.stack.push(child);
			}
			return Some(node);
		}
		None
	}
}
// endregion:	--- TreeIter

// region:		--- TreeIterMut
/// Mutable Iterator over the [`BehaviorTree`]
struct TeeIterMut<'a> {
	/// stack to do a depth first search
	stack: Vec<*mut BehaviorTreeElement>,
	/// Lifetime marker
	marker: PhantomData<&'a mut BehaviorTreeElement>,
}

impl<'a> TeeIterMut<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a mut Option<BehaviorTreeElement>) -> Self {
		Self {
			stack: vec![root.as_mut().expect("snh")],
			marker: PhantomData,
		}
	}
}

#[allow(unsafe_code)]
impl<'a> Iterator for TeeIterMut<'a> {
	type Item = &'a mut BehaviorTreeElement;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(node_ptr) = self.stack.pop() {
			// we know this pointer is valid since the iterator owns the traversal
			let node = unsafe { &mut *node_ptr };
			// Push children in revers order to maintain left-to-right order
			for child in node.children_iter_mut().rev() {
				self.stack.push(child);
			}
			return Some(&mut *node);
		}
		None
	}
}
// endregion:	--- TreeIterMut

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeElement`]s.
/// A certain [`BehaviorTree`] can contain up to 65536 [`BehaviorTreeElement`]s.
pub struct BehaviorTree {
	/// `root` beeing an [`Option`] allows to extract `root` temporarily
	/// and hand it ower to a spawned task.
	root: Option<BehaviorTreeElement>,
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
			root: Some(root),
			runtime,
			_libraries: libraries,
		}
	}

	/// Pretty print the tree.
	/// # Errors
	/// - if tree depth exceeds 127 (sub)tree levels.
	/// # Panics
	/// - if tree has no root
	pub fn print(&self) -> Result<(), Error> {
		print_recursively(0, self.root.as_ref().expect("snh"))
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
		let root = self.root.as_mut().expect("snh");

		root.execute_tick(&self.runtime).await
	}

	/// Ticks the tree once.
	/// @TODO: The wakeup mechanism is not yet implemented
	/// # Errors
	/// # Panics
	/// - if tree has no root
	pub async fn tick_once(&mut self) -> BehaviorResult {
		let root = self.root.as_mut().expect("snh");

		root.execute_tick(&self.runtime).await
	}

	/// Ticks the tree until it finishes either with [`BehaviorState::Success`] or [`BehaviorState::Failure`].
	/// # Errors
	/// # Panics
	/// - if tree has no root
	/// 
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		// will become #[cfg(feature = "std")]
		#[cfg(feature = "spawn")] 
		{
			let root = self.root.take();
			let runtime = self.runtime.clone();
			if let Some(mut task_root) = root {
				match tokio::spawn(async move {
					let mut state = BehaviorState::Running;
					while state == BehaviorState::Running || state == BehaviorState::Idle {
						state = match task_root.execute_tick(&runtime).await {
								Ok(state) => state,
								Err(err) => return (Err(err), task_root),
							};
						// Not implemented: Check for wake-up conditions and tick again if so
					}
					// halt eventually still running tasks
					match task_root.execute_halt(&runtime).await {
						Ok(()) => {},
						Err(err) => return (Err(err), task_root),
					};
					(Ok(state), task_root)
				}).await {
					Ok((result, root)) => {
						self.root.replace(root);
						result
					},
					Err(err) => {
						Err(BehaviorError::JoinError(err.to_string().into()))	
					},
				}
			} else {
				Err(BehaviorError::NoRoot)
			}
		}

		// will become #[cfg(not(feature = "std"))]
		#[cfg(not(feature = "spawn"))] 
		{
			let root = self.root.as_mut().expect("snh");
			let mut state = BehaviorState::Running;
			while state == BehaviorState::Running || state == BehaviorState::Idle {
				state = root.execute_tick(&self.runtime).await?;
				// Not implemented: Check for wake-up conditions and tick again if so
			}
			// halt eventually still running tasks
			root.execute_halt(&self.runtime).await?;
			// allow pending tasks to catch up
			tokio::task::yield_now().await;
			Ok(state)
		}
	}

	/// @TODO:
	pub fn iter(&self) -> impl Iterator<Item = &BehaviorTreeElement> {
		TreeIter::new(self.root.as_ref())
	}

	/// @TODO:
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BehaviorTreeElement> {
		TeeIterMut::new(&mut self.root)
	}
}
// endregion:	--- BehaviorTree
