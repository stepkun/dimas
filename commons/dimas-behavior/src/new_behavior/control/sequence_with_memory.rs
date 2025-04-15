// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! `SequenceWithMemory` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{
	new_behavior::{
		BehaviorCreation, BehaviorCreationFn, BehaviorMethods, BehaviorResult, BehaviorTickData,
		NewBehaviorStatus, NewBehaviorType, error::NewBehaviorError,
	},
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
#[derive(Debug)]
pub struct SequenceWithMemory;

impl BehaviorMethods for SequenceWithMemory {
	fn tick(&self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mut failure = false;
		let mut tick_data = tree_node.tick_data.lock();

		if tick_data.status == NewBehaviorStatus::Idle {
			tick_data.all_skipped = true;
		}

		tick_data.status = NewBehaviorStatus::Running;

		let children = tree_node.children.lock();
		while tick_data.child_idx < children.len() {
			let child = &children[tick_data.child_idx];
			let prev_status = child.status();
			let new_status = child.execute_tick()?;

			tick_data.all_skipped &= new_status == NewBehaviorStatus::Skipped;

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
					tick_data.child_idx += 1;
				}
			}
		};
		drop(children);

		if failure {
			// Do NOT reset children on failure
			// Halt children at and after current index
			tree_node.halt_children(tick_data.child_idx)?;
		} 
		// All children returned Success
		else if tick_data.child_idx == tree_node.children.lock().len() {
			// Reset children
			tree_node.reset_children()?;
			tick_data.child_idx = 0;
		}

		if tick_data.all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Failure)
		}
	}
}

impl BehaviorCreation for SequenceWithMemory {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}
// endregion:   --- Sequence
