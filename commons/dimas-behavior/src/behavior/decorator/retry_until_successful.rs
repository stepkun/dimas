// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType, error::BehaviorError},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- RetryUntilSuccessful
/// The `RetryUntilSuccessful` decorator is used to execute a child several times if it fails.
///
/// If the child returns SUCCESS, the loop is stopped and this node
/// returns SUCCESS.
///
/// If the child returns FAILURE, this decorator will try again up to N times
/// (N is read from port `num_attempts`).
///
/// In contrast to the `Retry` decorator, this decorator is non-reactive and does all attempts within 1 tick.
///
/// Example:
///
/// ```xml
/// <RetryUntilSuccessful num_attempts="3">
///     <OpenDoor/>
/// </RetryUntilSuccessful>
/// ```
#[derive(Behavior, Debug)]
pub struct RetryUntilSuccessful {
	/// Defaults to `-1`
	max_attempts: i32,
	/// Defaults to `0`
	try_count: i32,
	/// Defaults to `true`
	all_skipped: bool,
}

impl Default for RetryUntilSuccessful {
	fn default() -> Self {
		Self {
			max_attempts: -1,
			try_count: 0,
			all_skipped: true,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for RetryUntilSuccessful {
	async fn tick(
		&mut self,
		state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// Load num_cycles from the port value
		self.max_attempts = blackboard.get::<i32>("num_attempts".into())?;

		let mut do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;

		if state == BehaviorState::Idle {
			self.all_skipped = true;
		}

		while do_loop {
			// A `Decorator` has only 1 child
			let child = &mut children[0];
			let new_state = child.execute_tick(runtime).await?;

			self.all_skipped &= new_state == BehaviorState::Skipped;

			match new_state {
				BehaviorState::Failure => {
					self.try_count += 1;
					do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;
					children.reset(runtime)?;
				}
				BehaviorState::Idle => {
					return Err(BehaviorError::State("RetryUntilSuccessful".into(), "Idle".into()));
				}
				BehaviorState::Running => return Ok(BehaviorState::Running),
				BehaviorState::Skipped => {
					children.reset(runtime)?;
					return Ok(BehaviorState::Skipped);
				}
				BehaviorState::Success => {
					children.reset(runtime)?;
					self.try_count = 0;
					return Ok(BehaviorState::Success);
				}
			}
		}

		self.try_count = 0;

		if self.all_skipped {
			Ok(BehaviorState::Skipped)
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for RetryUntilSuccessful {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(i32, "num_attempts")]
	}
}
// endregion:   --- RetryUntilSuccessful
