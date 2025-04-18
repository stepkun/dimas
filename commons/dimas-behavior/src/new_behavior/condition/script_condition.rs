// Copyright © 2025 Stephan Kunz

//! Built in scripted condition behavior of `DiMAS`

// region:      --- modules
use crate::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType,
	},
	new_port::{NewPortList, input_port},
	tree::BehaviorTreeComponent,
};
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use dimas_behavior_derive::Behavior;
use dimas_scripting::{Parser, VM};
//endregion:    --- modules

/// The Script behavior returns Success or Failure depending on the result of the scripted code
#[derive(Behavior, Debug)]
pub struct ScriptCondition {}

impl BehaviorCreationMethods for ScriptCondition {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Condition
	}
}

impl BehaviorInstanceMethods for ScriptCondition {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let code = tree_node
			.tick_data
			.lock()
			.get_input::<String>("code")?;

		let mut parser = Parser::new(&code);
		let chunk = parser.parse()?;

		let env = tree_node.tick_data.lock().blackboard.clone();
		let mut vm = VM::default();
		let mut out = Vec::new();
		let value = vm.run(&chunk, &env, &mut out)?;

		let status = if value.is_bool() {
			let val = value.as_bool()?;
			if val {
				NewBehaviorStatus::Success
			} else {
				NewBehaviorStatus::Failure
			}
		} else {
			NewBehaviorStatus::Failure
		};

		Ok(status)
	}
}

impl BehaviorStaticMethods for ScriptCondition {
	fn provided_ports() -> NewPortList {
		vec![
			input_port::<String>(
				"code",
				"",
				"Piece of code that can be parsed. Must return false or true.",
			)
			.expect("snh"),
		]
	}
}
