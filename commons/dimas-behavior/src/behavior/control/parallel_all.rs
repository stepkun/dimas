// Copyright © 2025 Stephan Kunz

//! `ParallelAll` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use alloc::collections::btree_set::BTreeSet;
use dimas_scripting::SharedRuntime;

use crate::blackboard::BlackboardInterface;
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	port::PortList,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- ParallelAll
/// A `ParallelAll` executes its children
///
#[derive(Behavior, Debug)]
pub struct ParallelAll {
	/// @TODO:
	failure_threshold: i32,
	/// @TODO:
	completed_list: BTreeSet<usize>,
	/// @TODO:
	failure_count: usize,
}

impl Default for ParallelAll {
	fn default() -> Self {
		Self {
			failure_threshold: -1,
			completed_list: BTreeSet::default(),
			failure_count: 0,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for ParallelAll {
	async fn halt(
		&mut self,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		children.halt(0, runtime)
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn start(
		&mut self,
		state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// check composition only once at start
		self.failure_threshold = blackboard
			.get("max_failures".into())
			.unwrap_or(-1_i32);

		if (children.len() as i32) < self.failure_threshold {
			return Err(BehaviorError::Composition(
				"Number of children is less than the threshold. Can never fail.".into(),
			));
		}

		self.tick(state, blackboard, children, runtime)
			.await
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let children_count = children.len();

		let mut skipped_count = 0;

		for i in 0..children_count {
			// Skip completed node
			if self.completed_list.contains(&i) {
				continue;
			}

			let state = children[i].execute_tick(runtime).await?;
			match state {
				BehaviorState::Success => {
					self.completed_list.insert(i);
				}
				BehaviorState::Failure => {
					self.completed_list.insert(i);
					self.failure_count += 1;
				}
				BehaviorState::Skipped => skipped_count += 1,
				BehaviorState::Running => {}
				// Throw error, should never happen
				BehaviorState::Idle => {
					return Err(BehaviorError::State("ParallelAll".into(), "Idle".into()));
				}
			}
		}

		if skipped_count == children_count {
			return Ok(BehaviorState::Skipped);
		}

		if skipped_count + self.completed_list.len() >= children_count {
			// Done!
			children.reset(runtime)?;
			self.completed_list.clear();

			let state = if self.failure_count >= self.failure_threshold(children_count as i32) {
				BehaviorState::Failure
			} else {
				BehaviorState::Success
			};

			// Reset failure_count after using it
			self.failure_count = 0;

			return Ok(state);
		}

		Ok(BehaviorState::Running)
	}
}

impl BehaviorStatic for ParallelAll {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(i32, "max_failures", "1")]
	}
}
impl ParallelAll {
	#[allow(clippy::cast_sign_loss)]
	fn failure_threshold(&self, n_children: i32) -> usize {
		if self.failure_threshold < 0 {
			(n_children + self.failure_threshold + 1).max(0) as usize
		} else {
			self.failure_threshold as usize
		}
	}
}
// endregion:   --- ParallelAll
