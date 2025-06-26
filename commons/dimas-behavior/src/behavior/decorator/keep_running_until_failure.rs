// Copyright © 2025 Stephan Kunz

//! `KeepRunningUntilFailure` behavior implementation
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

// region:      --- KeepRunningUntilFailure
/// The `KeepRunningUntilFailure` decorator is used to execute a child several times if it fails.
///
///
/// Example:
///
/// ```xml
/// <KeepRunningUntilFailure>
///     <OpenDoor/>
/// </KeepRunningUntilFailure>
/// ```
#[derive(Behavior, Debug, Default)]
pub struct KeepRunningUntilFailure;

#[async_trait::async_trait]
impl BehaviorInstance for KeepRunningUntilFailure {
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		behavior.set_state(BehaviorState::Running);

		match children[0].execute_tick(runtime).await? {
			BehaviorState::Failure => {
				children.reset(runtime)?;
				Ok(BehaviorState::Failure)
			}
			BehaviorState::Idle => Err(BehaviorError::Composition(
				"KeepRunningUntilFailure should never return 'Idle'".into(),
			)),
			BehaviorState::Running => Ok(BehaviorState::Running),
			BehaviorState::Skipped => Err(BehaviorError::Composition(
				"KeepRunningUntilFailure should never return 'Skipped'".into(),
			)),
			BehaviorState::Success => {
				children.reset(runtime)?;
				Ok(BehaviorState::Running)
			}
		}
	}
}

impl BehaviorStatic for KeepRunningUntilFailure {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}
}
// endregion:   --- KeepRunningUntilFailure
