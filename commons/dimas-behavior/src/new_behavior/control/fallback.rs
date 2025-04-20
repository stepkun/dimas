// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString};
use dimas_behavior_derive::Behavior;

use crate::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType, error::NewBehaviorError,
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

impl BehaviorInstanceMethods for Fallback {
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		if tree_node.tick_data.status == NewBehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tree_node.tick_data.status = NewBehaviorStatus::Running;

		while self.child_idx < tree_node.children.len() {
			let child = &mut tree_node.children[self.child_idx];
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
					tree_node.reset_children()?;
					self.child_idx = 0;
					return Ok(NewBehaviorStatus::Success);
				}
			}
		}

		if self.child_idx >= tree_node.children.len() {
			tree_node.reset_children()?;
			self.child_idx = 0;
		}

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

impl BehaviorStaticMethods for Fallback {
	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}
// endregion:   --- Fallback
