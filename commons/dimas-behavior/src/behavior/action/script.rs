// Copyright © 2025 Stephan Kunz

//! Built in scripted action behavior of `DiMAS`

// region:      --- modules
use alloc::{boxed::Box, string::String};
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
//endregion:    --- modules

/// The `Script` behavior returns Success or Failure depending on the result of the scripted code.
#[derive(Behavior, Debug, Default)]
pub struct Script;

#[async_trait::async_trait]
impl BehaviorInstance for Script {
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let code = blackboard.get::<String>("code")?;
		let mut env = blackboard.clone();
		let value = runtime.lock().run(&code, &mut env)?;

		let state = if value.is_bool() {
			let val = value.as_bool()?;
			if val {
				BehaviorState::Success
			} else {
				BehaviorState::Failure
			}
		} else {
			BehaviorState::Success
		};

		Ok(state)
	}
}

impl BehaviorStatic for Script {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"code",
			"",
			"Piece of code that can be parsed."
		)]
	}
}
