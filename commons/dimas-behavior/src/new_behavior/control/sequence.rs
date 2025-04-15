// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]
#![allow(unused)]

//! Sequence behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{
	new_behavior::{
		BehaviorCreationFn, BehaviorMethods, BehaviorResult, BehaviorTickData, NewBehaviorStatus,
		error::NewBehaviorError,
	},
	tree::BehaviorTreeComponent,
};
// endregeion:  --- modules

// region:      --- Sequence
/// A Sequence ticks its children in an ordered sequence from first to last.
/// - If any child returns [`BehaviorStatus::Failure`] the sequence returns [`BehaviorStatus::Failure`].
/// - If all children return [`BehaviorStatus::Success`] the sequence returns [`BehaviorStatus::Success`].
/// - While any childr returns [`BehaviorStatus::Running`] the sequence returns [`BehaviorStatus::Running`].
///
/// While running, the loop is not restarted, first the running child will be ticked again.
/// If that tick succeeds the sequence continues, children that already succeeded will not be ticked again.
#[derive(Debug)]
pub struct Sequence;

impl BehaviorMethods for Sequence {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut Vec<BehaviorTreeComponent>,
	) -> BehaviorResult {
		if tick_data.status == NewBehaviorStatus::Idle {
			tick_data.all_skipped = true;
		}

		tick_data.status = NewBehaviorStatus::Running;

		while tick_data.child_idx < children.len() {
			let child = &mut children[tick_data.child_idx];
			let prev_status = child.status();
			let new_status = child.execute_tick()?;

			tick_data.all_skipped &= new_status == NewBehaviorStatus::Skipped;

			match new_status {
				NewBehaviorStatus::Failure => {
					return Ok(NewBehaviorStatus::Failure);
				}
				NewBehaviorStatus::Idle => {
					return Err(NewBehaviorError::Status(
						"SequenceNode".to_string(),
						"Idle".to_string(),
					));
				}
				NewBehaviorStatus::Running => return Ok(NewBehaviorStatus::Running),
				NewBehaviorStatus::Skipped | NewBehaviorStatus::Success => {
					tick_data.child_idx += 1;
				}
			}
		}

		Ok(NewBehaviorStatus::Success)
	}
}

impl Sequence {
	/// Provide a creation function
	#[must_use]
	pub fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self))
	}
}
// endregion:   --- Sequence
