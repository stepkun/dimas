// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! using a `struct` instead of a `trait` to improve performance.
//!

// region:      --- modules
use alloc::{
	borrow::ToOwned, boxed::Box, string::String, sync::Arc, vec::{self, Vec}
};
use core::{any::{Any, TypeId}, ops::{Deref, DerefMut}};
use dimas_scripting::{Parser, VM};
use hashbrown::HashMap;
use parking_lot::Mutex;
use rustc_hash::FxBuildHasher;

use crate::{
	new_behavior::{
		error::NewBehaviorError, BehaviorConfigurationData, BehaviorResult, BehaviorTickData, BehaviorTreeMethods, NewBehaviorStatus, SubtreeCallee, SubtreeCaller
	},
	new_blackboard::NewBlackboard,
};

use super::error::Error;
// endregion:   --- modules

// region:      --- BehaviorTreeComponent
/// The non [`Behavior`] data of a [`BehaviorTreeComponent`]
#[derive(Debug)]
pub struct BehaviorTreeComponent {
	/// Data needed in every tick
	pub tick_data: BehaviorTickData,
	/// Children
	pub children: Vec<BehaviorTreeComponentContainer>,
}

impl BehaviorTreeComponent {
	/// Reset all children for single child components.
	/// # Errors
	pub fn reset_child(&mut self) -> BehaviorResult {
		self.halt_child(0)
	}

	/// Reset all children for multi child components.
	/// # Errors
	pub fn reset_children(&mut self) -> BehaviorResult {
		self.halt_children(0)
	}

	/// halt all children at and beyond `index`
	/// # Errors
	/// - if index is out of childrens bounds
	pub fn halt_children(&mut self, index: usize) -> BehaviorResult {
		if index > self.children.len() {
			return Err(NewBehaviorError::IndexOutOfBounds(index));
		}

		for child in &mut *self.children {
			child.execute_halt()?;
		}
		Ok(NewBehaviorStatus::Idle)
	}

	/// halt all children at `index`
	/// # Errors
	/// - if index is out of childrens bounds
	pub fn halt_child(&mut self, index: usize) -> BehaviorResult {
		if index > self.children.len() {
			return Err(NewBehaviorError::IndexOutOfBounds(index));
		}

		self.children[index].execute_halt()?;
		Ok(NewBehaviorStatus::Idle)
	}
}
// endregion:   --- BehaviorTreeComponentInner

// region:      --- BehaviorTreeComponentContainer
/// Component within the [`BehaviorTree`]
#[derive(Debug)]
pub struct BehaviorTreeComponentContainer {
	/// Behavior of this node
	pub(crate) behavior: Box<dyn BehaviorTreeMethods>,
	/// tick tree component data
	pub(crate) inner: BehaviorTreeComponent,
	/// Data needed on rare occasions
	pub config_data: BehaviorConfigurationData,
}

impl Deref for BehaviorTreeComponentContainer {
	type Target = BehaviorTreeComponent;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for BehaviorTreeComponentContainer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

/// Methods needed for running a [`BehaviorTree`]
impl BehaviorTreeComponentContainer {
	/// Constructor for a leaf
	#[must_use]
	pub fn create_leaf(
		behavior: Box<dyn BehaviorTreeMethods>,
		tick_data: BehaviorTickData,
		config_data: BehaviorConfigurationData,
	) -> Self {
		Self {
			behavior,
			inner: BehaviorTreeComponent {
				tick_data,
				children: Vec::default(),
			},
			config_data,
		}
	}

	/// Constructor for a node
	/// # Panics
	/// - if after `is_some()` == true an unwrap fails
	#[must_use]
	pub fn create_node(
		behavior: Box<dyn BehaviorTreeMethods>,
		tick_data: BehaviorTickData,
		children: Vec<Self>,
		config_data: BehaviorConfigurationData,
	) -> Self {
		Self {
			behavior,
			inner: BehaviorTreeComponent {
				tick_data,
				children,
			},
			config_data,
		}
	}

	/// Access the Blackboard
	#[must_use]
	pub fn blackboard(&self) -> NewBlackboard {
		self.inner.tick_data.blackboard.clone()
	}

	/// Method called to tick a node in the [`Tree`].
	/// # Errors
	#[allow(unsafe_code)]
	pub fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status;
		if status == NewBehaviorStatus::Idle {
			status = self.behavior.start(&mut self.inner)?;
		} else {
			status = self.behavior.tick(&mut self.inner)?;
		}
		self.set_status(status);
		Ok(status)
	}

	/// Method called to stop a node in the [`Tree`].
	/// # Errors
	pub fn execute_halt(&mut self) -> BehaviorResult {
		self.set_status(NewBehaviorStatus::Idle);
		self.behavior
			.as_mut()
			.halt(&mut self.inner)
	}

	/// Set status of component
	pub fn set_status(&mut self, status: NewBehaviorStatus) {
		self.tick_data.status = status;
	}

	/// Get current status of component
	#[must_use]
	pub fn status(&self) -> NewBehaviorStatus {
		self.tick_data.status
	}

	/// Minimize memory footprint
	pub fn shrink(&mut self) {
		self.tick_data.remappings.shrink_to_fit();
		self.children.shrink_to_fit();
	}
}
// endregion:	--- BehaviorTreeComponentContainer

// region:      --- BehaviorTree
/// Tree of [`TreeNode`]s
#[derive(Debug, Default)]
pub struct BehaviorTree {
	/// Index of the root node in the vec below.
	root_index: usize,
	/// Map of direct accessible [`BehaviorTreeComponent`]s. These are `SubTree`s
	subtrees: Vec<Arc<Mutex<SubtreeCallee>>>,
}

impl BehaviorTree {
	pub(crate) fn add(&mut self, subtree: Arc<Mutex<SubtreeCallee>>) {
		self.subtrees.push(subtree);
	}

	pub(crate) fn set_root_index(&mut self) {
		self.root_index = self.subtrees.len() - 1;
	}

	/// Access a subtree in the subtree list by index
	#[must_use]
	pub fn subtree(&self, index: usize) -> Arc<Mutex<SubtreeCallee>> {
		self.subtrees[index].clone()
	}

	/// Access the subtreee list
	#[must_use]
	pub const fn subtrees(&self) -> &Vec<Arc<Mutex<SubtreeCallee>>> {
		&self.subtrees
	}

	/// Ticks the tree until it finishes either with [`BehaviorStatus::Success`] or [`BehaviorStatus::Failure`]
	/// # Errors
	/// - if no root exists
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		let mut status = NewBehaviorStatus::Idle;

		let mut root = self.subtrees[self.root_index].clone();

		while status == NewBehaviorStatus::Idle || matches!(status, NewBehaviorStatus::Running) {
			status = root.lock().execute_tick()?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				//root.reset_status();
			}
		}

		Ok(status)
	}

	/// Ticks the tree exactly once
	/// # Errors
	/// - if no root exists
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.subtrees[self.root_index]
			.lock().execute_tick()
	}

	/// Get root of tree
	#[must_use]
	pub fn root_node(&self) -> Arc<Mutex<SubtreeCallee>> {
		self.subtrees[self.root_index].clone()
	}

	/// Find a subtree by id
	/// # Errors
	/// - if the subtree is not within of the tree
	pub fn find_by_name(&self, id: &str) -> Result<Arc<Mutex<SubtreeCallee>>, Error> {
		for sub in &self.subtrees {
			if sub.lock().id() == id {
				return Ok(sub.clone());
			}
		}
		Err(Error::SubtreeNotFound(id.into()))
	}
	
	pub(crate) fn link_subtrees(&self) -> Result<(), Error> {
		let subtree = self.subtree(self.root_index);
		for child in subtree.lock().children() {
			if (child.type_id() == TypeId::of::<SubtreeCaller>()) {
				//let caller = todo!();
				let id = "t";
					let target = self.find_by_name(id)?;
				child.status();
			}
		}	
		Ok(())
	}
}
// endregion:   --- BehaviorTree
