// Copyright © 2025 Stephan Kunz

//! `Fallback` behavior implementation
//!

// region:      --- modules
use alloc::string::String;
use alloc::vec::Vec;
use dimas_scripting::{Parser, VM};

use crate as dimas_behavior;
use crate::behavior::error::BehaviorError;
use crate::blackboard::BlackboardInterface;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	input_port,
	port::PortList,
	port_list,
	tree::{BehaviorTreeComponent, BehaviorTreeElementList},
};
// endregion:   --- modules

// region:      --- Precondition
/// The `Precondition` behavior is used to check a scripted condition before
/// executing its child.
#[derive(Behavior, Debug, Default)]
pub struct Precondition {
	parser: Parser,
	vm: VM,
	stdout: Vec<u8>,
}

impl BehaviorInstance for Precondition {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		let if_branch = blackboard.get::<String>("if".into())?;
		let if_chunk = self.parser.parse(&if_branch)?;
		let mut env = blackboard.clone();
		let value = self
			.vm
			.run(&if_chunk, &mut env, &mut self.stdout)?;

		let status = if value.is_bool() {
			let val = value.as_bool()?;
			let child = &mut children[0];
			if val {
				// tick child and return the resulting value
				child.execute_tick()?
			} else {
				// halt eventually running child
				child.execute_halt()?;
				let else_branch = blackboard.get::<String>("else".into())?;
				match else_branch.as_ref() {
					"Failure" => BehaviorStatus::Failure,
					"Idle" => BehaviorStatus::Idle,
					"Running" => BehaviorStatus::Running,
					"Skipped" => BehaviorStatus::Skipped,
					"Success" => BehaviorStatus::Success,
					_ => {
						let else_chunk = self.parser.parse(&else_branch)?;
						let value = self
							.vm
							.run(&else_chunk, &mut env, &mut self.stdout)?;
						if value.is_bool() {
							let val = value.as_bool()?;
							if val {
								BehaviorStatus::Success
							} else {
								BehaviorStatus::Failure
							}
						} else {
							return Err(BehaviorError::NotABool);
						}
					}
				}
			}
		} else {
			return Err(BehaviorError::NotABool);
		};

		Ok(status)
	}
}

impl BehaviorStatic for Precondition {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![
			input_port!(String, "if", "", "Condition to check."),
			input_port!(String, "else", "", "Return status if condition is false."),
		]
	}
}
// endregion:   --- Precondition
