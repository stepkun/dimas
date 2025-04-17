// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! `Sequence` behavior implementation
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

// region:      --- Sequence
/// A `Sequence` ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorStatus::Failure`] the sequence returns [`BehaviorStatus::Failure`].
/// - If all children return [`BehaviorStatus::Success`] the sequence returns [`BehaviorStatus::Success`].
/// - While any child returns [`BehaviorStatus::Running`] the sequence returns [`BehaviorStatus::Running`].
///
/// While running, the loop is not restarted, first the running child will be ticked again.
/// If that tick succeeds the sequence continues, children that already succeeded will not be ticked again.
#[derive(Behavior, Debug, Default)]
pub struct Sequence {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'false'
	all_skipped: bool,
}

impl BehaviorInstanceMethods for Sequence {
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
			let prev_status = child.status();
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					failure = true;
					break;
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"Sequence".to_string(),
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

		// All children returned Success or Failure
		if failure || self.child_idx == children_len {
			// Reset children
			tree_node.reset_children()?;
			self.child_idx = 0;
		}
		if failure {
			Ok(NewBehaviorStatus::Failure)
		} else {
			Ok(NewBehaviorStatus::Success)
		}
	}

	fn halt(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorCreationMethods for Sequence {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for Sequence {}
// endregion:   --- Sequence
