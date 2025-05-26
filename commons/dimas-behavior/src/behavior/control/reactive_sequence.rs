// Copyright © 2025 Stephan Kunz

//! `ReactiveSequence` behavior implementation
//!

// region:      --- modules
use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType, error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- ReactiveSequence
/// A `ReactiveSequence` ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorStatus::Failure`] the sequence returns [`BehaviorStatus::Failure`].
/// - If all children return [`BehaviorStatus::Success`] the sequence returns [`BehaviorStatus::Success`].
/// - While any child returns [`BehaviorStatus::Running`] the sequence returns [`BehaviorStatus::Running`].
///
/// If all the children return SUCCESS, this node returns SUCCESS.
///
/// IMPORTANT: to work properly, this node should not have more than a single
///            asynchronous child.
#[derive(Behavior, Debug, Default)]
pub struct ReactiveSequence {
	/// Defaults to 'false'
	running: bool,
	/// Defaults to '0'
	child_idx: usize,
}

impl BehaviorInstance for ReactiveSequence {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		let mut all_skipped = true;
		tick_data.set_status(BehaviorStatus::Running);

		for counter in 0..children.len() {
			let child = &mut children[counter];
			let new_status = child.execute_tick()?;

			all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {
					children.reset()?;
					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"ReactiveSequence".into(),
						"Idle".into(),
					));
				}
				BehaviorStatus::Running => {
					for i in 0..counter {
						children[i].execute_halt()?;
					}
					if !self.running {
						self.child_idx = counter;
						self.running = true;
					} else if self.child_idx != counter {
						// Multiple children running at the same time
						return Err(BehaviorError::Composition(
							"[ReactiveSequence]: Only a single child can return Running.".into(),
						));
					}
					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Skipped => {
					// halt current child
					child.execute_halt()?;
				}
				BehaviorStatus::Success => {}
			}
		}

		// Reset children on failure
		children.reset()?;
		if all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Success)
		}
	}
}

impl BehaviorStatic for ReactiveSequence {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- ReactiveSequence
