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
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mut failure = false;
		let mut tick_data = tree_node.tick_data.lock();

		if tick_data.status == NewBehaviorStatus::Idle {
			self.all_skipped = true;
		}

		tick_data.status = NewBehaviorStatus::Running;

		let children = tree_node.children.lock();
		let children_len = children.len();
		while self.child_idx < children_len {
			let child = &children[self.child_idx];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					failure = true;
					break;
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
		drop(children);
		drop(tick_data);

		if failure {
			// Do NOT reset children on failure
			// Halt children at and after current index
			tree_node.halt_children(self.child_idx)?;
		}
		// All children returned Success
		else if self.child_idx == children_len {
			// Reset children
			tree_node.reset_children()?;
			self.child_idx = 0;
		}

		if self.all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Failure)
		}
	}

	fn halt(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorCreationMethods for SequenceWithMemory {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for SequenceWithMemory {}
// endregion:   --- SequenceWithMemory
