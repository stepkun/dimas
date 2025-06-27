// Copyright © 2025 Stephan Kunz

//! `Repeat` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Repeat
/// The [`Repeat`] decorator is used to execute a child several times as long as it succeeds.
///
/// Example:
///
/// ```xml
/// <Repeat num_cycles="3">
///     <WaveHand/>
/// </Repeat>
/// ```
#[derive(Behavior, Debug)]
pub struct Repeat {
	/// Defaults to `-1`
	num_cycles: i32,
	/// Defaults to `0`
	repeat_count: i32,
}

impl Default for Repeat {
	fn default() -> Self {
		Self {
			num_cycles: -1,
			repeat_count: 0,
		}
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for Repeat {
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// Load num_cycles from the port value
		self.num_cycles = behavior.get::<i32>("num_cycles")?;
		self.repeat_count = 0;

		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		behavior.set_state(BehaviorState::Running);

		if self.repeat_count < self.num_cycles || self.num_cycles == -1 {
			let child = &mut children[0];
			let new_state = child.execute_tick(runtime).await?;

			match new_state {
				BehaviorState::Failure => {
					self.repeat_count = 0;
					children.reset(runtime).await?;
					Ok(BehaviorState::Failure)
				}
				BehaviorState::Idle => Err(BehaviorError::State("RetryUntilSuccessful".into(), "Idle".into())),
				BehaviorState::Running => return Ok(BehaviorState::Running),
				BehaviorState::Skipped => {
					children.reset(runtime).await?;
					Ok(BehaviorState::Skipped)
				}
				BehaviorState::Success => {
					self.repeat_count += 1;
					children.reset(runtime).await?;
					Ok(BehaviorState::Running)
				}
			}
		} else {
			Ok(behavior.state())
		}
	}
}

impl BehaviorStatic for Repeat {
	fn kind() -> BehaviorKind {
		BehaviorKind::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			i32,
			"num_cycles",
			"",
			"Repeat a successful child up to N times. Use -1 to create an infinite loop."
		)]
	}
}
// endregion:   --- RetryUntilSuccessful
