// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec};
use dimas_behavior_derive::Behavior;

use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType, error::BehaviorError,
	},
	input_port_macro,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
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

impl BehaviorInstanceMethods for RetryUntilSuccessful {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		// Load num_cycles from the port value
		self.max_attempts = tick_data.get_input("num_attempts")?;

		let mut do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;

		if tick_data.status == BehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tick_data.status = BehaviorStatus::Running;

		while do_loop {
			// A `Decorator` has only 1 child
			let child = &mut children[0];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == BehaviorStatus::Skipped;

			match new_status {
				BehaviorStatus::Failure => {
					self.try_count += 1;
					do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;
					children.reset()?;
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"RetryUntilSuccessful".to_string(),
						"Idle".to_string(),
					));
				}
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Skipped => {
					children.reset()?;
					return Ok(BehaviorStatus::Skipped);
				}
				BehaviorStatus::Success => {
					children.reset()?;
					self.try_count = 0;
					return Ok(BehaviorStatus::Success);
				}
			}
		}

		self.try_count = 0;

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}
}

impl BehaviorStaticMethods for RetryUntilSuccessful {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port_macro!(i32, "num_attempts")]
	}
}
// endregion:   --- RetryUntilSuccessful
