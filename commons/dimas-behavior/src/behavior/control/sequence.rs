// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType, error::BehaviorError,
	},
	port::PortList,
	tree::BehaviorTreeComponentList,
};
// endregion:   --- modules

// region:      --- Sequence
/// A `Sequence` ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorStatus::Failure`] the sequence returns [`BehaviorStatus::Failure`].
/// - If all children return [`BehaviorStatus::Success`] the sequence returns [`BehaviorStatus::Success`].
/// - While any child returns [`BehaviorStatus::Running`] the sequence returns [`BehaviorStatus::Running`].
///
/// While running, the loop is not restarted, first the running child will be ticked again.
/// If that tick succeeds the sequence continues, children that already succeeded will not be ticked again.
#[derive(Behavior, Debug, Default)]
pub struct Sequence {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'false'
	all_skipped: bool,
}

impl BehaviorInstanceMethods for Sequence {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		if tick_data.status == BehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tick_data.status = BehaviorStatus::Running;

		while self.child_idx < children.len() {
			let child = &mut children[self.child_idx];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {
					children.reset()?;
					self.child_idx = 0;
					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Sequence".into(),
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

impl BehaviorStaticMethods for Sequence {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Sequence
