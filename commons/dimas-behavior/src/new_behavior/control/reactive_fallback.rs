// Copyright © 2025 Stephan Kunz

//! `Sequence` behavior implementation
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

// region:      --- ReactiveFallback
/// The `ReactiveFallback` behavior is used to try different strategies until one succeeds,
/// but every strategy is re-evaluated on each tick.
/// All the children are ticked from first to last:
/// - If a child returns RUNNING, continue to the next sibling.
/// - If a child returns FAILURE, continue to the next sibling.
/// - If a child returns SUCCESS, stop and return SUCCESS.
///
/// If all the children fail, than this node returns FAILURE.
///
/// IMPORTANT: to work properly, this node should not have more than
///            a single asynchronous child.
#[derive(Behavior, Debug, Default)]
pub struct ReactiveFallback {
	/// Defaults to '0'
	child_idx: usize,
	/// Defaults to 'false'
	all_skipped: bool,
}

impl BehaviorInstanceMethods for ReactiveFallback {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mut success = false;
		let mut tick_data = tree_node.tick_data.lock();

		self.all_skipped = true;
		tick_data.status = NewBehaviorStatus::Running;

		let children = tree_node.children.lock();
		for index in 0..children.len() {
			let child = &children[self.child_idx];
			let new_status = child.execute_tick()?;

			self.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"ReactiveFallback".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => {
					for i in 0..index {
						let cd = &children[i];
						cd.execute_halt()?;
					}
					return Ok(NewBehaviorStatus::Running);
				}
				NewBehaviorStatus::Skipped => {
					child.execute_halt()?;
				}
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

impl BehaviorCreationMethods for ReactiveFallback {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Control
	}
}

impl BehaviorStaticMethods for ReactiveFallback {}
// endregion:   --- ReactiveFallback
