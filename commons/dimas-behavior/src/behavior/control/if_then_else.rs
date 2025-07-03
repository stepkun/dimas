// Copyright © 2025 Stephan Kunz

//! `IfThenElse` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- IfThenElse
/// The `IfThenElse` behavior must have exactly 2 or 3 children. This behavior is NOT reactive.
///
/// The first child is the "statement" of the if.
/// - If that return Success, then the second child is executed.
/// - Instead, if it returned Failure, the third child is executed.
///
/// If you have only 2 children, this node will return Failure whenever the
/// statement returns Failure.
/// This is equivalent to adding [`AlwaysFailure`] as 3rd child.
#[derive(Behavior, Debug, Default)]
pub struct IfThenElse {
	child_index: usize,
}

#[async_trait::async_trait]
impl BehaviorInstance for IfThenElse {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		children.reset(runtime).await?;
		self.child_index = 0;
		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}

	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// check composition only once at start
		if !(2..=3).contains(&children.len()) {
			return Err(BehaviorError::Composition(
				"IfThenElse must have either 2 or 3 children.".into(),
			));
		}

		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		behavior.set_state(BehaviorState::Running);

		let children_count = children.len();

		if self.child_index == 0 {
			let condition_state = children[0].execute_tick(runtime).await?;
			match condition_state {
				BehaviorState::Success => {
					self.child_index = 1;
				}
				BehaviorState::Failure => match children_count {
					3 => {
						self.child_index = 2;
					}
					2 => {
						return Ok(condition_state);
					}
					_ => {}
				},
				_ => {}
			}
		}

		// execute the branch
		if self.child_index > 0 {
			let state = children[self.child_index]
				.execute_tick(runtime)
				.await?;
			if state != BehaviorState::Running {
				children.reset(runtime).await?;
				self.child_index = 0;
			}
			Ok(state)
		} else {
			Err(BehaviorError::Composition(
				"Something unexpected happened in IfThenElse".into(),
			))
		}
	}
}

impl BehaviorStatic for IfThenElse {
	fn kind() -> BehaviorKind {
		BehaviorKind::Control
	}
}
// endregion:   --- IfThenElse
