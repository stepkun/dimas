// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec};
use dimas_behavior_derive::Behavior;

use crate::{
	input_port_macro,
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType, error::NewBehaviorError,
	},
	new_port::NewPortList,
	port_list,
	tree::BehaviorTreeComponent,
};
// endregeion:  --- modules

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
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		// Load num_cycles from the port value
		// @TODO: maybe this could be done in the default() method??
		self.max_attempts = tree_node.tick_data.get_input("num_attempts")?;

		let mut do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;

		if tree_node.tick_data.status == NewBehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tree_node.tick_data.status = NewBehaviorStatus::Running;

		while do_loop {
			// A `Decorator` has only 1 child
			let child = &mut tree_node.children[0];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					self.try_count += 1;
					do_loop = self.try_count < self.max_attempts || self.max_attempts == -1;
					tree_node.reset_child()?;
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"RetryUntilSuccessful".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
				NewBehaviorStatus::Skipped => {
					tree_node.reset_child()?;
					return Ok(NewBehaviorStatus::Skipped);
				}
				NewBehaviorStatus::Success => {
					tree_node.reset_child()?;
					self.try_count = 0;
					return Ok(NewBehaviorStatus::Success);
				}
			}
		}

		self.try_count = 0;

		if self.all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Failure)
		}
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorStaticMethods for RetryUntilSuccessful {
	fn kind() -> NewBehaviorType {
		NewBehaviorType::Decorator
	}

	fn provided_ports() -> NewPortList {
		port_list![input_port_macro!(i32, "num_attempts")]
	}
}
// endregion:   --- RetryUntilSuccessful
