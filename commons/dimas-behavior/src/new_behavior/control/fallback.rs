// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec::Vec};
use dimas_behavior_derive::Behavior;

use crate::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTickData,
		BehaviorTreeMethods, NewBehaviorStatus, NewBehaviorType, error::NewBehaviorError,
	},
	new_port::NewPortList,
	tree::BehaviorTreeComponent,
};
// endregeion:  --- modules

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

extern crate std;
impl BehaviorInstanceMethods for Fallback {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mut success = false;
		let mut tick_data = tree_node.tick_data.lock();
		if tick_data.status == NewBehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tick_data.status = NewBehaviorStatus::Running;

		let children = tree_node.children.lock();
		while self.child_idx < children.len() {
			let child = &children[self.child_idx];
			let prev_status = child.status();
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure | NewBehaviorStatus::Skipped => {
					self.child_idx += 1;
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"Fallback".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
				NewBehaviorStatus::Success => {
					success = true;
					break;
				}
			}
		}
		drop(children);
		drop(tick_data);

		tree_node.reset_children()?;
		self.child_idx = 0;

		if success {
			Ok(NewBehaviorStatus::Success)
		} else if self.all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Failure)
		}
	}

	fn halt(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorCreationMethods for Fallback {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for Fallback {}
// endregion:   --- Fallback
