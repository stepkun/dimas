// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in run-once node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_behavior::port::PortList;
use dimas_behavior::{define_ports, input_port};
use dimas_macros::behavior;
//endregion:    --- modules

/// The RunOnceNode is used when you want to execute the child
/// only once.
/// If the child is asynchronous, we will tick until either SUCCESS or FAILURE is
/// returned.
///
/// After that first execution, you can set value of the port "then_skip" to:
///
/// - if TRUE (default), the node will be skipped in the future.
/// - if FALSE, return synchronously the same status returned by the child, forever.
#[behavior(SyncDecorator)]
pub struct RunOnce {
	#[bhvr(default = "false")]
	already_ticked: bool,
	#[bhvr(default = "BehaviorStatus::Idle")]
	returned_status: BehaviorStatus,
}

#[behavior(SyncDecorator)]
impl RunOnce {
	async fn tick(&mut self) -> BehaviorResult {
		let skip = bhvr_.config_mut().get_input("then_skip")?;

		if self.already_ticked {
			return if skip {
				Ok(BehaviorStatus::Skipped)
			} else {
				Ok(self.returned_status.clone())
			};
		}

		bhvr_.set_status(BehaviorStatus::Running);

		let status = bhvr_
			.child()
			.unwrap_or_else(|| todo!())
			.execute_tick()
			.await?;

		if status.is_completed() {
			self.already_ticked = true;
			self.returned_status = status;
			bhvr_.reset_child().await;
		}

		Ok(status)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("then_skip", true))
	}

	async fn halt(&mut self) {
		bhvr_.reset_child().await;
	}
}
