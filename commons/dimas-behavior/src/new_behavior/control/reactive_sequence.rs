// Copyright © 2025 Stephan Kunz

//! `ReactiveSequence` behavior implementation
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
#[derive(Behavior, Debug, Default)]
pub struct ReactiveSequence {
	/// Defaults to 'false'
	running: bool,
	/// Defaults to '0'
	child_idx: usize,
}

impl BehaviorInstanceMethods for ReactiveSequence {
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let mut all_skipped = true;

		tree_node.tick_data.status = NewBehaviorStatus::Running;

		for counter in 0..tree_node.children.len() {
			let child = &mut tree_node.children[counter];
			let new_status = child.execute_tick()?;

			all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					tree_node.reset_children()?;
					return Ok(NewBehaviorStatus::Failure);
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"ReactiveSequence".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => {
					for i in 0..counter {
						tree_node.children[i].execute_halt()?;
					}
					if !self.running {
						self.child_idx = counter;
						self.running = true;
					} else if self.child_idx != counter {
						// Multiple children running at the same time
						return Err(NewBehaviorError::Composition(
							"[ReactiveSequence]: Only a single child can return Running."
								.to_string(),
						));
					}
					return Ok(NewBehaviorStatus::Running);
				}
				NewBehaviorStatus::Skipped => {
					// halt current child
					child.execute_halt()?;
				}
				NewBehaviorStatus::Success => {}
			}
		}

		// Reset children on failure
		tree_node.reset_children()?;
		if all_skipped {
			Ok(NewBehaviorStatus::Skipped)
		} else {
			Ok(NewBehaviorStatus::Success)
		}
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node.halt_children(0)
	}
}

impl BehaviorCreationMethods for ReactiveSequence {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for ReactiveSequence {}
// endregion:   --- ReactiveSequence
