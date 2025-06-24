// Copyright © 2025 Stephan Kunz

//! `ScriptPrecondition` behavior implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::String};
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::behavior::error::BehaviorError;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Precondition
/// The `Precondition` behavior is used to check a scripted condition before
/// executing its child.
#[derive(Behavior, Debug, Default)]
pub struct Precondition;

#[async_trait::async_trait]
impl BehaviorInstance for Precondition {
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let if_branch = behavior.get::<String>("if")?;
		let value = runtime
			.lock()
			.run(&if_branch, behavior.blackboard_mut())?;

		let new_state = if value.is_bool() {
			let val = value.as_bool()?;
			let child = &mut children[0];
			if val {
				// tick child and return the resulting value
				child.execute_tick(runtime).await?
			} else {
				// halt eventually running child
				child.execute_halt(runtime).await?;
				let else_branch = behavior.get::<String>("else")?;
				match else_branch.as_ref() {
					"Failure" => BehaviorState::Failure,
					"Idle" => BehaviorState::Idle,
					"Running" => BehaviorState::Running,
					"Skipped" => BehaviorState::Skipped,
					"Success" => BehaviorState::Success,
					_ => {
						let value = runtime
							.lock()
							.run(&else_branch, behavior.blackboard_mut())?;
						if value.is_bool() {
							let val = value.as_bool()?;
							if val {
								BehaviorState::Success
							} else {
								BehaviorState::Failure
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

		Ok(new_state)
	}
}

impl BehaviorStatic for Precondition {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![
			input_port!(String, "if", "", "Condition to check."),
			input_port!(String, "else", "", "Return state if condition is false."),
		]
	}
}
// endregion:   --- Precondition
