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
	tick_data: BehaviorTickData,
	/// Children
	children: Vec<BehaviorTreeComponent>,
}

/// Methods needed for running a [`BehaviorTree`]
impl BehaviorTreeComponent {
	/// Constructor for a leaf
	#[must_use]
	pub fn create_leaf(behavior: Box<dyn BehaviorMethods>, tick_data: BehaviorTickData) -> Self {
		Self {
			behavior: Some(behavior),
			tick_data,
			children: Vec::default(),
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
			tick_data,
			children,
		}
	}

	/// Method called to tick a node in the [`Tree`].
	/// # Errors
	pub fn execute_tick(&mut self) -> BehaviorResult {
		if let Some(bhvr) = self.behavior.as_deref_mut() {
			if self.tick_data.status == NewBehaviorStatus::Idle {
				let result = bhvr.start(&mut self.tick_data, &mut self.children);
				if let Ok(status) = result {
					if status.is_completed() {
						self.reset_children();
					}
				}
				result
			} else {
				let result = bhvr.tick(&mut self.tick_data, &mut self.children);
				if let Ok(status) = result {
					if status.is_completed() {
						self.reset_children();
					}
				}
				result
			}
		} else {
			for mut child in &mut self.children {
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
	pub fn execute_halt(&mut self) -> BehaviorResult {
		if let Some(bhvr) = &mut self.behavior {
			bhvr.halt(&mut self.tick_data, &mut self.children)
		} else {
			Ok(NewBehaviorStatus::Idle)
		}
	}

	/// Set status of component
	pub fn set_status(&mut self, status: NewBehaviorStatus) {
		self.tick_data.status = status;
	}

	/// Get current status of component
	#[must_use]
	pub const fn status(&self) -> NewBehaviorStatus {
		self.tick_data.status
	}

	/// reset all children
	pub fn reset_children(&mut self) {
		self.halt_children(0);
	}

	/// halt all children
	/// # Errors
	/// - if index is out of childrens bounds
	pub fn halt_children(&mut self, index: usize) -> Result<(), Error> {
		if index > self.children.len() {
			return Err(Error::IndexOutOfBounds(index));
		}

		for child in &mut self.children {
			child.execute_halt()?;
		}
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
