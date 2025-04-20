// Copyright © 2025 Stephan Kunz

//! Built in scripted action behavior of `DiMAS`

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
#[derive(Behavior, Debug, Default)]
pub struct Script {
	parser: Parser,
}

impl BehaviorInstanceMethods for Script {
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let code = tree_node.tick_data.get_input::<String>("code")?;

		let chunk = self.parser.parse(&code)?;

		let env = tree_node.tick_data.blackboard.clone();
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
			NewBehaviorStatus::Success
		};

		Ok(status)
	}
}

impl BehaviorStaticMethods for Script {
	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}

	fn provided_ports() -> NewPortList {
		vec![input_port::<String>("code", "", "Piece of code that can be parsed.").expect("snh")]
	}
}
