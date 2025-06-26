// Copyright © 2025 Stephan Kunz

//! Built in [`Sleep`] action behavior of `DiMAS`

// region:      --- modules
use alloc::boxed::Box;
use core::time::Duration;
use dimas_scripting::SharedRuntime;
#[cfg(feature = "std")]
use tokio::task::JoinHandle;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::tree::BehaviorTreeElementList;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	input_port,
	port::PortList,
	port_list,
};
//endregion:    --- modules

// region:		--- Sleep
/// The [`Sleep`] behavior sleeps for the amount of time given via port msec.
/// Consider also using the decorator [`Delay`]
#[derive(Behavior, Debug, Default)]
pub struct Sleep {
	handle: Option<JoinHandle<()>>,
}

#[async_trait::async_trait]
impl BehaviorInstance for Sleep {
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
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		if let Some(handle) = self.handle.as_ref() {
			if handle.is_finished() {
				self.handle = None;
				Ok(BehaviorState::Success)
			} else {
				Ok(BehaviorState::Running)
			}
		} else {
			Ok(BehaviorState::Failure)
		}
	}
}

impl BehaviorStatic for Sleep {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			u64,
			"msec",
			"",
			"Time to sleep in [msec]."
		)]
	}
}
// endregion:	--- Sleep
