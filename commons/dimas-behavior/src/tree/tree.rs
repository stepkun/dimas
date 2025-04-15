// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! using a `struct` insteaf of a `trait` to improve performance.
//!

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use hashbrown::HashMap;
use parking_lot::Mutex;

use crate::{
	blackboard::Blackboard,
	new_behavior::{
		BehaviorMethods, BehaviorResult, BehaviorTickData, NewBehaviorStatus, NewBehaviorType,
		control::ControlBehavior, error::NewBehaviorError,
	},
	tree::error::Error,
};
// endregion:   --- modules

// region:      --- BehaviorTreeComponent
/// Component within the [`BehaviorTree`]
#[derive(Debug)]
pub struct BehaviorTreeComponent {
	/// Behavior of this node
	behavior: Option<Box<dyn BehaviorMethods>>,
	/// Data needed in every tick
	pub tick_data: Mutex<BehaviorTickData>,
	/// Children
	pub children: Mutex<Vec<BehaviorTreeComponent>>,
}

/// Methods needed for running a [`BehaviorTree`]
impl BehaviorTreeComponent {
	/// Constructor for a leaf
	#[must_use]
	pub fn create_leaf(behavior: Box<dyn BehaviorMethods>, tick_data: BehaviorTickData) -> Self {
		Self {
			behavior: Some(behavior),
			tick_data: Mutex::new(tick_data),
			children: Mutex::new(Vec::default()),
		}
	}

	/// Constructor for a node
	#[must_use]
	pub fn create_node(
		behavior: Option<Box<dyn BehaviorMethods>>,
		tick_data: BehaviorTickData,
		children: Vec<Self>,
	) -> Self {
		Self {
			behavior,
			tick_data: Mutex::new(tick_data),
			children: Mutex::new(children),
		}
	}

	/// Method called to tick a node in the [`Tree`].
	/// # Errors
	#[allow(unsafe_code)]
	pub fn execute_tick(&self) -> BehaviorResult {
		if let Some(bhvr) = &self.behavior {
			if self.tick_data.lock().status == NewBehaviorStatus::Idle {
				bhvr.start(self)
			} else {
				bhvr.tick(self)
			}
		} else {
			for mut child in &*self.children.lock() {
				match child.execute_tick()? {
					NewBehaviorStatus::Success => continue,
					NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
					NewBehaviorStatus::Failure => return Ok(NewBehaviorStatus::Failure),
					NewBehaviorStatus::Idle => todo!(),
					NewBehaviorStatus::Skipped => todo!(),
				}
			}
			Ok(NewBehaviorStatus::Success)
		}
	}

	/// Method called to stop a node in the [`Tree`].
	/// # Errors
	pub fn execute_halt(&self) -> BehaviorResult {
		self.behavior
			.as_ref()
			.map_or(Ok(NewBehaviorStatus::Idle), |bhvr| bhvr.halt(self))
	}

	/// Set status of component
	pub fn set_status(&mut self, status: NewBehaviorStatus) {
		self.tick_data.lock().status = status;
	}

	/// Get current status of component
	#[must_use]
	pub fn status(&self) -> NewBehaviorStatus {
		self.tick_data.lock().status
	}

	/// reset all children
	/// # Errors
	pub fn reset_children(&self) -> Result<(), NewBehaviorError> {
		self.halt_children(0)
	}

	/// halt all children at and beyond `index`
	/// # Errors
	/// - if index is out of childrens bounds
	pub fn halt_children(&self, index: usize) -> Result<(), NewBehaviorError> {
		if index > self.children.lock().len() {
			return Err(NewBehaviorError::IndexOutOfBounds(index));
		}

		for child in &*self.children.lock() {
			child.execute_halt()?;
		}
		Ok(())
	}

	/// halt all children at `index`
	/// # Errors
	/// - if index is out of childrens bounds
	pub fn halt_child(&self, index: usize) -> Result<(), NewBehaviorError> {
		if index > self.children.lock().len() {
			return Err(NewBehaviorError::IndexOutOfBounds(index));
		}

		self.children.lock()[index].execute_halt()?;
		Ok(())
	}
}

// region:		--- TickOption
#[repr(u8)]
enum TickOption {
	WhileRunning,
	ExactlyOnce,
	OnceUnlessWokenUp,
}
// endregeion:	--- TickOption

// region:      --- BehaviorTree
/// Tree of [`TreeNode`]s
#[derive(Debug, Default)]
pub struct BehaviorTree {
	/// Id of the root node in the map below.
	root_id: String,
	/// Map of direct accessible [`BehaviorTreeNode`]s
	subtrees: HashMap<String, BehaviorTreeComponent>,
}

impl BehaviorTree {
	pub(crate) fn add(&mut self, id: &str, subtree: BehaviorTreeComponent) {
		self.subtrees.insert(id.into(), subtree);
	}

	pub(crate) fn set_root_id(&mut self, id: &str) {
		self.root_id = id.into();
	}

	/// Ticks the tree until it finishes either with [`BehaviorStatus::Success`] or [`BehaviorStatus::Failure`]
	/// # Errors
	/// - if no root exists
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::WhileRunning).await
	}

	async fn tick_root(&mut self, opt: TickOption) -> BehaviorResult {
		let mut status = NewBehaviorStatus::Idle;

		let mut root = self
			.subtrees
			.get_mut(&self.root_id)
			.ok_or_else(|| NewBehaviorError::RootNotFound(self.root_id.clone()))?;

		while status == NewBehaviorStatus::Idle
			|| (matches!(opt, TickOption::WhileRunning)
				&& matches!(status, NewBehaviorStatus::Running))
		{
			status = root.execute_tick()?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				//root.reset_status();
			}
		}

		Ok(status)
	}
}
// endregion:   --- BehaviorTree
