// Copyright © 2025 Stephan Kunz

//! `SequenceWithMemory` behavior implementation
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
				NewBehaviorStatus::Failure => {
					// Do NOT reset children on failure
					// Halt children at and after current index
					tree_node.halt_children(self.child_idx)?;
					return Ok(NewBehaviorStatus::Failure);
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"SequenceWithMemory".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
				NewBehaviorStatus::Skipped | NewBehaviorStatus::Success => {
					self.child_idx += 1;
				}
			}
		}

		// All children returned Success
		if self.child_idx >= tree_node.children.len() {
			// Reset children
			tree_node.reset_children()?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Success)
		}
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorStaticMethods for SequenceWithMemory {
	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}
// endregion:   --- SequenceWithMemory
