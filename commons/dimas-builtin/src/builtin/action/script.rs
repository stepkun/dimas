// Copyright © 2025 Stephan Kunz

//! Built in scripted action behavior of `DiMAS`

// region:      --- modules
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use dimas_behavior::{
	behavior::{BehaviorResult, BehaviorStatus},
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::behavior;
use dimas_scripting::{Parser, VM};
//endregion:    --- modules

/// The Script behavior returns Success or Failure depending on the result of the scripted code
#[behavior(SyncAction)]
pub struct Script {}

extern crate std;

#[behavior(SyncAction)]
impl Script {
	fn ports() -> PortList {
		define_ports!(input_port!("code", "Piece of code that can be parsed"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let code = bhvr_.config_mut().get_input::<String>("code")?;
		let mut parser = Parser::new();
		let chunk = parser.parse(&code)?;

		let env = bhvr_.config().blackboard();
		let mut vm = VM::default();
		let mut out = Vec::new();
		let value = vm.run(&chunk, env, &mut out)?;
		let status = if value.is_bool() {
			let val = value.as_bool()?;
			if val {
				BehaviorStatus::Success
			} else {
				BehaviorStatus::Failure
			}
		} else {
			BehaviorStatus::Success
		};

		Ok(status)
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
