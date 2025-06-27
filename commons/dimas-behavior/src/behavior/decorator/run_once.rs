// Copyright © 2025 Stephan Kunz

//! `RunOnce` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- RunOnce
/// The [`RunOnce`] decorator .
#[derive(Behavior, Debug, Default)]
pub struct RunOnce {
	already_ticked: bool,
	then_skip: bool,
	state: BehaviorState,
}

#[async_trait::async_trait]
impl BehaviorInstance for RunOnce {
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.then_skip = behavior.get::<bool>("then_skip")?;

		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		if self.already_ticked {
			if self.then_skip {
				Ok(BehaviorState::Skipped)
			} else {
				Ok(self.state)
			}
		} else {
			behavior.set_state(BehaviorState::Running);
			let state = children[0].execute_tick(runtime).await?;
			if state.is_completed() {
				self.already_ticked = true;
				self.state = state;
				children.reset(runtime).await?;
			}
			Ok(state)
		}
	}
}

impl BehaviorStatic for RunOnce {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			bool,
			"then_skip",
			"true",
			"If true, skip after the first execution, otherwise return the same 'BehaviorState' returned once by the child"
		)]
	}
}
// endregion:   --- RetryUntilSuccessful
