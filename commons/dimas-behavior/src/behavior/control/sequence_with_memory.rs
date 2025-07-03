// Copyright © 2025 Stephan Kunz

//! `SequenceWithMemory` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
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
#[derive(Behavior, Debug)]
pub struct SequenceWithMemory {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'true'
	all_skipped: bool,
}

impl Default for SequenceWithMemory {
	fn default() -> Self {
		Self {
			child_idx: 0,
			all_skipped: true,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for SequenceWithMemory {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		children.reset(runtime).await?;
		self.child_idx = 0;
		self.all_skipped = true;
		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		behavior.set_state(BehaviorState::Running);

		while self.child_idx < children.len() {
			let child = &mut children[self.child_idx];
			let new_state = child.execute_tick(runtime).await?;

			self.all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {
					// Do NOT reset children on failure
					// Halt children at and after current index
					children.halt(self.child_idx, runtime)?;
					return Ok(BehaviorState::Failure);
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State("SequenceWithMemory".into(), "Idle".into()));
				}
				BehaviorState::Running => return Ok(BehaviorState::Running),
				BehaviorState::Skipped | BehaviorState::Success => {
					self.child_idx += 1;
				}
			}
		}

		// All children returned Success
		if self.child_idx >= children.len() {
			// Reset children
			children.reset(runtime).await?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Success)
		}
	}
}

impl BehaviorStatic for SequenceWithMemory {
	fn kind() -> BehaviorKind {
		BehaviorKind::Control
	}
}
// endregion:   --- SequenceWithMemory
