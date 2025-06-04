// Copyright © 2025 Stephan Kunz

//! `Parallel` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;
use hashbrown::HashSet;

use crate::behavior::error::BehaviorError;
use crate::blackboard::BlackboardInterface;
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::SharedBlackboard,
	port::PortList,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Parallel
/// A `Parallel` ticks executes children in
///
#[derive(Behavior, Debug)]
pub struct Parallel {
	/// @TODO:
	success_threshold: i32,
	/// @TODO:
	failure_threshold: i32,
	/// @TODO:
	completed_list: HashSet<usize>,
	/// @TODO:
	success_count: usize,
	/// @TODO:
	failure_count: usize,
}

impl Default for Parallel {
	fn default() -> Self {
		Self {
			success_threshold: -1,
			failure_threshold: -1,
			completed_list: HashSet::default(),
			success_count: 0,
			failure_count: 0,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for Parallel {
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn tick(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.success_threshold = blackboard
			.get("success_count".into())
			.unwrap_or(-1);
		self.failure_threshold = blackboard
			.get("failure_count".into())
			.unwrap_or(-1);

		let children_count = children.len();

		if children_count < self.success_threshold(children_count as i32) {
			return Err(BehaviorError::Composition(
				#[allow(clippy::match_same_arms)]
				"Number of children is less than the threshold. Can never succeed.".into(),
			));
		}

		if children_count < self.failure_threshold(children_count as i32) {
			return Err(BehaviorError::Composition(
				"Number of children is less than the threshold. Can never fail.".into(),
			));
		}

		let mut skipped_count = 0;

		for i in 0..children_count {
			if !self.completed_list.contains(&i) {
				let child = &mut children[i];
				match child.execute_tick(runtime).await? {
					BehaviorState::Skipped => skipped_count += 1,
					BehaviorState::Success => {
						self.completed_list.insert(i);
						self.success_count += 1;
					}
					BehaviorState::Failure => {
						self.completed_list.insert(i);
						self.failure_count += 1;
					}
					BehaviorState::Running => {}
					// Throw error, should never happen
					BehaviorState::Idle => {
						todo!()
					}
				}
			}

			let required_success_count = self.success_threshold(children_count as i32);

			// Check if success condition has been met
			if self.success_count >= required_success_count
				|| (self.success_threshold < 0
					&& (self.success_count + skipped_count) >= required_success_count)
			{
				self.clear();
				children.reset(runtime)?;
				return Ok(BehaviorState::Success);
			}

			if (children_count - self.failure_count) < required_success_count
				|| self.failure_count == self.failure_threshold(children_count as i32)
			{
				self.clear();
				children.reset(runtime)?;
				return Ok(BehaviorState::Failure);
			}
		}

		// If all children were skipped, return Skipped
		// Otherwise return Running
		if skipped_count == children_count {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Running)
		}
	}
}

impl BehaviorStatic for Parallel {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}

	fn provided_ports() -> PortList {
		port_list![
			input_port!(i32, "failure_count"),
			input_port!(i32, "success_count")
		]
	}
}
impl Parallel {
	#[allow(clippy::cast_sign_loss)]
	fn success_threshold(&self, n_children: i32) -> usize {
		if self.success_threshold < 0 {
			(n_children + self.success_threshold + 1).max(0) as usize
		} else {
			self.success_threshold as usize
		}
	}

	#[allow(clippy::cast_sign_loss)]
	fn failure_threshold(&self, n_children: i32) -> usize {
		if self.failure_threshold < 0 {
			(n_children + self.failure_threshold + 1).max(0) as usize
		} else {
			self.failure_threshold as usize
		}
	}

	fn clear(&mut self) {
		self.completed_list.clear();
		self.success_count = 0;
		self.failure_count = 0;
	}
}
// endregion:   --- Parallel
