// Copyright © 2025 Stephan Kunz

//! Built in scripted condition behavior of `DiMAS`

// region:      --- modules
use crate::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType,
	},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port_macro,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
};
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::any::Any;
use dimas_behavior_derive::Behavior;
use dimas_scripting::{Parser, VM};
//endregion:    --- modules

/// The Script behavior returns Success or Failure depending on the result of the scripted code
#[derive(Behavior, Debug, Default)]
pub struct ScriptCondition {
	parser: Parser,
}

impl BehaviorInstanceMethods for ScriptCondition {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let code = blackboard.get::<String>("code".into())?;

		let chunk = self.parser.parse(&code)?;

		let mut env = blackboard.clone();
		let mut vm = VM::default();
		let mut out = Vec::new();
		let value = vm.run(&chunk, &mut env, &mut out)?;

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

impl BehaviorStaticMethods for ScriptCondition {
	fn kind() -> BehaviorType {
		BehaviorType::Condition
	}

	fn provided_ports() -> PortList {
		port_list![input_port_macro!(
			String,
			"code",
			"",
			"Piece of code that can be parsed. Must return false or true."
		)]
	}
}
