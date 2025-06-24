// Copyright © 2025 Stephan Kunz

//! `ForceFailure` behavior implementation
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

// region:      --- Inverter
/// The `Inverter` behavior is used to try different strategies until one succeeds.
/// If any child returns RUNNING, previous children will NOT be ticked again.
/// - If all the children return FAILURE, this node returns FAILURE.
/// - If a child returns RUNNING, this node returns RUNNING.
/// - If a child returns SUCCESS, stop the loop and return SUCCESS.
#[derive(Behavior, Debug, Default)]
pub struct ForceFailure {}

#[async_trait::async_trait]
impl BehaviorInstance for ForceFailure {
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let child = &mut children[0];
		let new_state = child.execute_tick(runtime).await?;

		match new_state {
			BehaviorState::Failure => {
				children.reset(runtime)?;
				Ok(BehaviorState::Failure)
			}
			BehaviorState::Idle => Err(BehaviorError::State("ForceFailure".into(), "Idle".into())),
			state @ (BehaviorState::Running | BehaviorState::Skipped) => Ok(state),
			BehaviorState::Success => {
				children.reset(runtime)?;
				Ok(BehaviorState::Failure)
			}
		}
	}
}

impl BehaviorStatic for ForceFailure {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}
}
// endregion:   --- Inverter
