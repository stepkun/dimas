// Copyright © 2025 Stephan Kunz

//! `Updated` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use alloc::string::String;
use dimas_core::ConstString;
use dimas_scripting::SharedRuntime;

use crate::behavior::{BehaviorData, BehaviorError};
use crate::port::{PortList, is_bb_pointer, strip_bb_pointer};
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- WasEntryUpdated
/// The `WasEntryUpdated` condition returns Success if a blackboard entry was updated otherwise Failure.
#[derive(Behavior, Debug, Default)]
pub struct WasEntryUpdated {
	/// ID of the last checked update
	sequence_id: usize,
	/// The entry to monitor
	entry_key: ConstString,
}

#[async_trait::async_trait]
impl BehaviorInstance for WasEntryUpdated {
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.sequence_id = 0;
		if let Some(key) = behavior.remappings.find(&"entry".into()) {
			if is_bb_pointer(&key) {
				let stripped = strip_bb_pointer(&key).expect("snh");
				self.entry_key = behavior.get::<String>(&stripped)?.into();
			} else {
				self.entry_key = behavior.get::<String>(&key)?.into();
			}
			self.tick(behavior, children, runtime).await
		} else {
			Err(BehaviorError::PortNotDeclared(
				"entry".into(),
				behavior.description().name().clone(),
			))
		}
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		if let Ok(sequence_id) = behavior.get_sequence_id(&self.entry_key) {
			if sequence_id == self.sequence_id {
				Ok(BehaviorState::Failure)
			} else {
				Ok(BehaviorState::Success)
			}
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for WasEntryUpdated {
	fn kind() -> BehaviorKind {
		BehaviorKind::Condition
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"entry",
			"",
			"The blackboard entry to check."
		)]
	}
}
// endregion:   --- WasEntryUpdated
