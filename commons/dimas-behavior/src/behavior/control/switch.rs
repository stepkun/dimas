// Copyright © 2025 Stephan Kunz

//! `Switch` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate::behavior::BehaviorData;
use crate::port::PortList;
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Switch
/// The `Switch` behavior is .
#[derive(Behavior, Debug)]
pub struct Switch {
	/// Defaults to '-1'
	running_child_idx: i32,
}

impl Default for Switch {
	fn default() -> Self {
		Self { running_child_idx: -1 }
	}
}

#[async_trait::async_trait]
impl BehaviorInstance for Switch {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		for child in &mut **children {
			child.execute_halt(runtime).await?;
		}
		self.running_child_idx = -1;

		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}

	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.running_child_idx = -1;
		self.tick(behavior, children, runtime).await
	}
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		let match_index = -1;

		// stop child, if it is not the one that should run
		if match_index != self.running_child_idx && match_index >= 0 {
			#[allow(clippy::cast_sign_loss)]
			children[match_index as usize]
				.execute_halt(runtime)
				.await?;
		}
		Ok(BehaviorState::Failure)
	}
}

impl BehaviorStatic for Switch {
	fn kind() -> BehaviorKind {
		BehaviorKind::Control
	}

	fn provided_ports() -> PortList {
		port_list![
			input_port!(i32, "failure_count"),
			input_port!(i32, "success_count")
		]
	}
}
// endregion:   --- Fallback
