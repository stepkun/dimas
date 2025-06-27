// Copyright © 2025 Stephan Kunz

//! `Updated` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_core::ConstString;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Updated
/// The `Updated` behavior is .
#[derive(Behavior, Debug, Default)]
pub struct Updated {
	/// ID of the last checked update
	sequence_id: usize,
	/// Still running the child
	is_running: bool,
	/// What to return if key is not updated
	state_if_not: BehaviorState,
	/// The entry to monitor
	entry_key: ConstString,
}

#[async_trait::async_trait]
impl BehaviorInstance for Updated {
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.sequence_id = 0;
		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		if self.is_running {
			let state = children[0].execute_tick(runtime).await?;
			self.is_running = state == BehaviorState::Running;
			return Ok(state);
		}

		let sequence_id = behavior.get_sequence_id(&self.entry_key)?;
		if sequence_id == self.sequence_id {
			Ok(self.state_if_not)
		} else {
			self.sequence_id = sequence_id;
			Ok(children[0].execute_tick(runtime).await?)
		}
	}
}

impl BehaviorStatic for Updated {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}
}
// endregion:   --- Updated
