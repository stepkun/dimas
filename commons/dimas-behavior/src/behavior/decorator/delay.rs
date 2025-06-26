// Copyright © 2025 Stephan Kunz

//! Built in [`Delay`] decorator of `DiMAS`

// region:      --- modules
use alloc::boxed::Box;
use core::time::Duration;
use dimas_scripting::SharedRuntime;
#[cfg(feature = "std")]
use tokio::task::JoinHandle;

use crate as dimas_behavior;
use crate::behavior::{BehaviorData, BehaviorError};
use crate::tree::BehaviorTreeElementList;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	input_port,
	port::PortList,
	port_list,
};
//endregion:    --- modules

// region:		--- Delay
/// The [`Delay`] decorator will introduce a delay given by the port `delay_msec` and then tick its child.
/// Consider also using the action [`Sleep`]
#[derive(Behavior, Debug, Default)]
pub struct Delay {
	handle: Option<JoinHandle<()>>,
}

#[async_trait::async_trait]
impl BehaviorInstance for Delay {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		for child in &mut **children {
			child.halt(0, runtime)?;
		}
		self.handle = None;
		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}

	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let millis: u64 = behavior.get("msec")?;
		self.handle = Some(tokio::task::spawn(async move {
			tokio::time::sleep(Duration::from_millis(millis)).await;
		}));

		Ok(BehaviorState::Running)
	}

	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		if let Some(handle) = self.handle.as_ref() {
			if handle.is_finished() {
				let state = children[0].execute_tick(runtime).await?;
				if state.is_completed() {
					children.reset(runtime)?;
					Ok(BehaviorState::Success)
				} else {
					Ok(state)
				}
			} else {
				Ok(BehaviorState::Running)
			}
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for Delay {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			u64,
			"delay_msec",
			"",
			"Tick the child after a few milliseconds."
		)]
	}
}
// endregion:	--- Delay
