// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! using a `struct` instead of a `trait` to improve performance.
//!

use core::ops::{Deref, DerefMut};

// region:      --- modules
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use dimas_scripting::{Parser, VM};
use hashbrown::HashMap;
use parking_lot::Mutex;
use rustc_hash::FxBuildHasher;

use crate::new_behavior::{
	BehaviorConfigurationData, BehaviorResult, BehaviorTickData, BehaviorTreeMethods,
	NewBehaviorStatus, error::NewBehaviorError,
};
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
	behavior: Option<Box<dyn BehaviorTreeMethods>>,
	/// tick tree component data
	inner: BehaviorTreeComponent,
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
			behavior: Some(behavior),
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
		behavior: Option<Box<dyn BehaviorTreeMethods>>,
		tick_data: BehaviorTickData,
		children: Vec<Self>,
		config_data: BehaviorConfigurationData,
	) -> Self {
		let behavior = behavior.map_or_else(|| None, Some);
		Self {
			behavior,
			inner: BehaviorTreeComponent {
				tick_data,
				children,
			},
			config_data,
		}
	}

	/// Method called to tick a node in the [`Tree`].
	/// # Errors
	#[allow(unsafe_code)]
	pub fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status;
		if let Some(bhvr) = &mut self.behavior {
			if status == NewBehaviorStatus::Idle {
				status = bhvr.start(&mut self.inner)?;
			} else {
				status = bhvr.tick(&mut self.inner)?;
			}
		} else {
			for child in &mut *self.children {
				match child.execute_tick()? {
					NewBehaviorStatus::Success => {}
					NewBehaviorStatus::Running => {
						self.set_status(NewBehaviorStatus::Running);
						return Ok(NewBehaviorStatus::Running);
					}
					NewBehaviorStatus::Failure => {
						self.set_status(NewBehaviorStatus::Failure);
						return Ok(NewBehaviorStatus::Failure);
					}
					NewBehaviorStatus::Idle => todo!(),
					NewBehaviorStatus::Skipped => todo!(),
				}
			}
			status = NewBehaviorStatus::Success;
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
			.map_or(Ok(NewBehaviorStatus::Idle), |bhvr| {
				bhvr.halt(&mut self.inner)
			})
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
	/// @TODO: replace with a vec and maybe use references
	subtrees: Vec<BehaviorTreeComponentContainer>,
}

impl BehaviorTree {
	pub(crate) fn add(&mut self, subtree: BehaviorTreeComponentContainer) {
		self.subtrees.push(subtree);
	}

	pub(crate) fn set_root_index(&mut self) {
		self.root_index = self.subtrees.len() - 1;
	}

	/// Ticks the tree until it finishes either with [`BehaviorStatus::Success`] or [`BehaviorStatus::Failure`]
	/// # Errors
	/// - if no root exists
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		let mut status = NewBehaviorStatus::Idle;

		let root = &mut self.subtrees[self.root_index];

		while status == NewBehaviorStatus::Idle || matches!(status, NewBehaviorStatus::Running) {
			status = root.execute_tick()?;

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
		self.subtrees[self.root_index].execute_tick()
	}

	/// Get root of tree
	#[must_use]
	pub fn root_node(&self) -> &BehaviorTreeComponentContainer {
		&self.subtrees[self.root_index]
	}
}
// endregion:   --- BehaviorTree
