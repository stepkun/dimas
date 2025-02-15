// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in parallel-all node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_core::port::PortList;
use dimas_core::{define_ports, input_port};
use dimas_macros::behavior;
use hashbrown::HashSet;
//endregion:    --- modules

/// The ParallelAllNode execute all its children
/// __concurrently__, but not in separate threads!
///
/// It differs in the way ParallelNode works because the latter may stop
/// and halt other children if a certain number of SUCCESS/FAILURES is reached,
/// whilst this one will always complete the execution of ALL its children.
///
/// Note that threshold indexes work as in Python: [see](https://www.i2tutorials.com/what-are-negative-indexes-and-why-are-they-used/)
///
/// Therefore -1 is equivalent to the number of children.
#[behavior(SyncControl)]
pub struct ParallelAll {
	#[bhvr(default = "-1")]
	failure_threshold: i32,
	#[bhvr(default)]
	completed_list: HashSet<usize>,
	#[bhvr(default = "0")]
	failure_count: usize,
}

#[behavior(SyncControl)]
impl ParallelAll {
	#[allow(clippy::cast_sign_loss)]
	fn failure_threshold(&self, n_children: i32) -> usize {
		if self.failure_threshold < 0 {
			(n_children + self.failure_threshold + 1).max(0) as usize
		} else {
			self.failure_threshold as usize
		}
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	async fn tick(&mut self) -> BehaviorResult {
		self.failure_threshold = bhvr_.config_mut().get_input("max_failures")?;

		let children_count = bhvr_.children().len();

		if (children_count as i32) < self.failure_threshold {
			return Err(BehaviorError::Composition(
				"Number of children is less than the threshold. Can never fail.".to_string(),
			));
		}

		let mut skipped_count = 0;

		for i in 0..children_count {
			// Skip completed node
			if self.completed_list.contains(&i) {
				continue;
			}

			let status = bhvr_.children_mut()[i].execute_tick().await?;
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
						"ParallelAllNode".to_string(),
						"Idle".to_string(),
					));
				}
			}
		}

		if skipped_count == children_count {
			return Ok(BehaviorStatus::Skipped);
		}

		if skipped_count + self.completed_list.len() >= children_count {
			// Done!
			bhvr_.reset_children().await;
			self.completed_list.clear();

			let status =
				if self.failure_count >= self.failure_threshold(bhvr_.children().len() as i32) {
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

	fn ports() -> PortList {
		define_ports!(input_port!("max_failures", 1))
	}

	async fn halt(&mut self) {
		bhvr_.reset_children().await;
	}
}
