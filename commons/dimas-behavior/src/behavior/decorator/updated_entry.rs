// Copyright © 2025 Stephan Kunz

//! `Updated` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use dimas_core::ConstString;
use dimas_scripting::SharedRuntime;

use crate::port::PortList;
use crate::{self as dimas_behavior, input_port, port_list};
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
pub struct UpdatedEntry {
	/// ID of the last checked update
	sequence_id: usize,
	/// Still running the child
	is_running: bool,
	/// What to return if key is not updated
	state_if_not: BehaviorState,
	/// The entry to monitor
	entry_key: ConstString,
}

impl UpdatedEntry {
	pub(crate) fn new(state: BehaviorState) -> Self {
		Self { 
			sequence_id: 0, 
			is_running: false, 
			state_if_not: state, 
			entry_key: Arc::default()
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for UpdatedEntry {
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.sequence_id = 0;
		self.entry_key = behavior.get::<String>("entry")?.into();
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

		if let Ok(sequence_id) = behavior.get_sequence_id(&self.entry_key) {
			if sequence_id == self.sequence_id {
				Ok(self.state_if_not)
			} else {
				self.sequence_id = sequence_id;
				let state = children[0].execute_tick(runtime).await?;
				self.is_running = state == BehaviorState::Running;
				return Ok(state);
			}
		} else {
			Ok(self.state_if_not)
		}
	}
}

impl BehaviorStatic for UpdatedEntry {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"entry",
			"",
			"The blackboard entry to monitor."
		)]
	}
}
// endregion:   --- Updated
