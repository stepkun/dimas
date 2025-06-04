// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
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

#[async_trait::async_trait]
impl BehaviorInstance for Fallback {
	async fn tick(
		&mut self,
		state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		if state == BehaviorState::Idle {
			self.all_skipped = true;
		}

		while self.child_idx < children.len() {
			let child = &mut children[self.child_idx];
			let new_state = child.execute_tick(runtime).await?;

			self.all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure | BehaviorState::Skipped => {
					self.child_idx += 1;
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State("Fallback".into(), "Idle".into()));
				}
				BehaviorState::Running => return Ok(BehaviorState::Running),
				BehaviorState::Success => {
					children.reset(runtime)?;
					self.child_idx = 0;
					return Ok(BehaviorState::Success);
				}
			}
		}

		if self.child_idx >= children.len() {
			children.reset(runtime)?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for Fallback {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Fallback
