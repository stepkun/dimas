// Copyright © 2025 Stephan Kunz

//! `ParallelAll` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use hashbrown::HashSet;

use crate::blackboard::BlackboardInterface;
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorType,
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
	completed_list: HashSet<usize>,
	/// @TODO:
	failure_count: usize,
}

impl Default for ParallelAll {
	fn default() -> Self {
		Self {
			failure_threshold: -1,
			completed_list: HashSet::default(),
			failure_count: 0,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for ParallelAll {
	async fn halt(&mut self, children: &mut BehaviorTreeElementList) -> Result<(), BehaviorError> {
		children.halt(0)
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn tick(
		&mut self,
		_status: BehaviorStatus,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		self.failure_threshold = blackboard.get("max_failures".into()).unwrap_or(-1_i32);

		let children_count = children.len();
		if (children_count as i32) < self.failure_threshold {
			return Err(BehaviorError::Composition(
				"Number of children is less than the threshold. Can never fail.".into(),
			));
		}

		let mut skipped_count = 0;

		for i in 0..children_count {
			// Skip completed node
			if self.completed_list.contains(&i) {
				continue;
			}

			let status = children[i].execute_tick().await?;
			match status {
				BehaviorStatus::Success => {
					self.completed_list.insert(i);
				}
				BehaviorStatus::Failure => {
					self.completed_list.insert(i);
					self.failure_count += 1;
				}
				BehaviorStatus::Skipped => skipped_count += 1,
				BehaviorStatus::Running => {}
				// Throw error, should never happen
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"ParallelAll".into(),
						"Idle".into(),
					));
				}
			}
		}

		if skipped_count == children_count {
			return Ok(BehaviorStatus::Skipped);
		}

		if skipped_count + self.completed_list.len() >= children_count {
			// Done!
			children.reset()?;
			self.completed_list.clear();

			let status =
				if self.failure_count >= self.failure_threshold(children_count as i32) {
					BehaviorStatus::Failure
				} else {
					BehaviorStatus::Success
				};

			// Reset failure_count after using it
			self.failure_count = 0;

			return Ok(status);
		}

		Ok(BehaviorStatus::Running)
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
