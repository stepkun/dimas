// Copyright © 2025 Stephan Kunz

//! Built in scripted condition behavior of `DiMAS`

// region:      --- modules
use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
};
use alloc::{string::String, vec::Vec};
use dimas_scripting::{Parser, VM};
//endregion:    --- modules

/// The `ScriptCondition` behavior returns Success or Failure depending on the result of the scripted code.
#[derive(Behavior, Debug, Default)]
pub struct ScriptCondition {
	parser: Parser,
	vm: VM,
	stdout: Vec<u8>,
}

impl BehaviorInstance for ScriptCondition {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let code = blackboard.get::<String>("code".into())?;
		let chunk = self.parser.parse(&code)?;
		let mut env = blackboard.clone();
		let value = self.vm.run(&chunk, &mut env, &mut self.stdout)?;

		let status = if value.is_bool() {
			let val = value.as_bool()?;
			if val {
				BehaviorStatus::Success
			} else {
				BehaviorStatus::Failure
			}
		} else {
			BehaviorStatus::Failure
		};

		Ok(status)
	}
}

impl BehaviorStatic for ScriptCondition {
	fn kind() -> BehaviorType {
		BehaviorType::Condition
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"code",
			"",
			"Piece of code that can be parsed. Must return false or true."
		)]
	}
}
