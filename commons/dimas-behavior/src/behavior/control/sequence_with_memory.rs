// Copyright © 2025 Stephan Kunz

//! `SequenceWithMemory` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use core::any::Any;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType, error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	port::PortList,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentList},
};
// endregion:   --- modules

// region:      --- SequenceWithMemory
/// A `SequenceWithMemory` ticks its children in an ordered sequence from first to last.
/// If any child returns RUNNING, previous children are not ticked again.
/// - If all the children return SUCCESS, this node returns SUCCESS.
/// - If a child returns RUNNING, this node returns RUNNING.
///   Loop is NOT restarted, the same running child will be ticked again.
/// - If a child returns FAILURE, stop the loop and return FAILURE.
///
///   Loop is NOT restarted, the same running child will be ticked again.
#[derive(Behavior, Debug, Default)]
pub struct SequenceWithMemory {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'false'
	all_skipped: bool,
}

impl BehaviorInstanceMethods for SequenceWithMemory {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		if tick_data.status() == BehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tick_data.set_status(BehaviorStatus::Running);

		while self.child_idx < children.len() {
			let child = &mut children[self.child_idx];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {
					// Do NOT reset children on failure
					// Halt children at and after current index
					children.halt(self.child_idx)?;
					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"SequenceWithMemory".into(),
						"Idle".into(),
					));
				}
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Skipped | BehaviorStatus::Success => {
					self.child_idx += 1;
				}
			}
		}

		// All children returned Success
		if self.child_idx >= children.len() {
			// Reset children
			children.reset()?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Success)
		}
	}
}

impl BehaviorStaticMethods for SequenceWithMemory {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- SequenceWithMemory
