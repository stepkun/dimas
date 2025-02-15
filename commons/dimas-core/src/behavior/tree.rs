// Copyright © 2024 Stephan Kunz

//! `dimas` behavior tree

#[doc(hidden)]
extern crate alloc;

// @TODO: try to remove std lib

// region:      --- modules
use crate::{
	behavior::{Behavior, BehaviorResult, BehaviorStatus},
	blackboard::Blackboard,
};
use alloc::vec;
use alloc::vec::Vec;
// endregion:   --- modules

// region: 		--- BehaviorIter
/// Iterator over the [`BehaviorTree`]
/// @TODO:
struct BehaviorIter<'a> {
	nodes: Vec<&'a Behavior>,
	idxs: Vec<i32>,
}

impl<'a> BehaviorIter<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a Behavior) -> Self {
		Self {
			nodes: vec![root],
			idxs: vec![-1],
		}
	}
}

impl<'a> Iterator for BehaviorIter<'a> {
	type Item = &'a Behavior;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::cast_possible_wrap)]
	fn next(&mut self) -> Option<Self::Item> {
		// Loop until we find a node to return
		loop {
			// Out of nodes; we have traversed the entire tree
			if self.nodes.is_empty() {
				return None;
			}

			let end_idx = self.nodes.len() - 1;

			let node = self.nodes[end_idx];
			let child_idx = &mut self.idxs[end_idx];

			// When this index is -1, that means we haven't returned the node yet
			if *child_idx < 0 {
				self.idxs[end_idx] = 0;
				return Some(node);
			} else if node.children().is_none()
				|| *child_idx >= node.children().unwrap_or_else(|| todo!()).len() as i32
			{
				// When the node has no children, pop it off and try the next element
				// OR
				// If we've already returned all children, pop it off
				// Unwrap is safe because we just checked if it's None
				self.nodes.pop();
				self.idxs.pop();
			} else {
				// If nothing else applies, we can push the node's child and return it
				// Unwrap is safe because we just checked if it's None
				let child = &node.children().unwrap_or_else(|| todo!())[*child_idx as usize];
				*child_idx += 1;

				self.nodes.push(child);
				self.idxs.push(-1);
			}
		}
	}
}
// endregion:	--- BehaviorIter

// region:      --- BehaviorTree
enum TickOption {
	WhileRunning,
	ExactlyOnce,
	OnceUnlessWokenUp,
}

/// The behavior node tree
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct BehaviorTree {
	root: Behavior,
}

impl BehaviorTree {
	/// @TODO:
	#[must_use]
	pub const fn new(root: Behavior) -> Self {
		Self { root }
	}

	async fn tick_root(&mut self, opt: TickOption) -> BehaviorResult {
		let mut status = BehaviorStatus::Idle;

		while status == BehaviorStatus::Idle
			|| (matches!(opt, TickOption::WhileRunning)
				&& matches!(status, BehaviorStatus::Running))
		{
			status = self.root.execute_tick().await?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				self.root.reset_status();
			}
		}

		Ok(status)
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_exactly_once(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::ExactlyOnce).await
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::OnceUnlessWokenUp)
			.await
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::WhileRunning).await
	}

	/// @TODO:
	#[must_use]
	pub fn root_blackboard(&self) -> Blackboard {
		self.root.config().blackboard().clone()
	}

	/// @TODO:
	pub async fn halt_tree(&mut self) {
		self.root.halt().await;
	}

	/// @TODO:
	pub fn visit_nodes(&self) -> impl Iterator<Item = &Behavior> {
		BehaviorIter::new(&self.root)
	}
}
// endregion:   --- BehaviorTree
