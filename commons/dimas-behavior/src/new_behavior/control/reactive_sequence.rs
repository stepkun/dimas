// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! `ReactiveSequence` behavior implementation
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

// region:      --- ReactiveSequence
/// A `ReactiveSequence` ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorStatus::Failure`] the sequence returns [`BehaviorStatus::Failure`].
/// - If all children return [`BehaviorStatus::Success`] the sequence returns [`BehaviorStatus::Success`].
/// - While any child returns [`BehaviorStatus::Running`] the sequence returns [`BehaviorStatus::Running`].
///
/// If all the children return SUCCESS, this node returns SUCCESS.
///
/// IMPORTANT: to work properly, this node should not have more than a single
///            asynchronous child.
#[derive(Debug)]
pub struct ReactiveSequence;

impl BehaviorMethods for ReactiveSequence {
	fn tick(&self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mut all_skipped = true;
		let mut failure = false;

		let mut tick_data = tree_node.tick_data.lock();

		tick_data.status = NewBehaviorStatus::Running;

		let mut children = tree_node.children.lock();
		for counter in 0..children.len() {
			let mut child = &children[counter];
			let prev_status = child.status();
			let new_status = child.execute_tick()?;

			all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					failure = true;
					break;
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"ReactiveSequence".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
				NewBehaviorStatus::Skipped => { child.execute_halt()?; },
				NewBehaviorStatus::Success => { continue; }
			};
		}
		drop(children);
		drop(tick_data);

		// Reset children on failure
		tree_node.reset_children()?;

		if failure {
			Ok(NewBehaviorStatus::Failure)
		} else if all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Success)
		}
	}
}

impl BehaviorCreation for ReactiveSequence {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}
// endregion:   --- Sequence
