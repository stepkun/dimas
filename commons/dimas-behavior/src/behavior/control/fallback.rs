// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString};
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

// region:      --- Fallback
/// The `Fallback` behavior is used to try different strategies until one succeeds.
/// If any child returns RUNNING, previous children will NOT be ticked again.
/// - If all the children return FAILURE, this node returns FAILURE.
/// - If a child returns RUNNING, this node returns RUNNING.
/// - If a child returns SUCCESS, stop the loop and return SUCCESS.
#[derive(Behavior, Debug, Default)]
pub struct Fallback {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'false'
	all_skipped: bool,
}

impl BehaviorInstanceMethods for Fallback {
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
				BehaviorStatus::Failure | BehaviorStatus::Skipped => {
					self.child_idx += 1;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Fallback".to_string(),
						"Idle".to_string(),
					));
				}
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Success => {
					children.reset()?;
					self.child_idx = 0;
					return Ok(BehaviorStatus::Success);
				}
			}
		}

		if self.child_idx >= children.len() {
			children.reset()?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}
}

impl BehaviorStaticMethods for Fallback {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Fallback
