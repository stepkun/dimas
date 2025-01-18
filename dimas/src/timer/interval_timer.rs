// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::unused_async)]
#![allow(missing_docs)]

//! Interval timer

// region:      --- modules
use anyhow::Result;
use core::time::Duration;
use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{Behavior, BehaviorCategory, BehaviorResult, BehaviorStatus},
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_decorator};
use tokio::{task::JoinHandle, time};
// endregion:   --- modules

// region:      --- behavior
/// An [`IntervalTimer`]
#[behavior(SyncDecorator)]
pub struct IntervalTimer {
	/// The handle to stop the Timer
	#[bhvr(default = "None")]
	handle: Option<JoinHandle<()>>,
}

#[behavior(SyncDecorator)]
impl IntervalTimer {
	async fn tick(&self) -> BehaviorResult {
		println!("IntervalTimer");

		// timer already started?
		if self.handle.is_none() {
			bhvr_.set_status(BehaviorStatus::Running);

			let input = bhvr_.config.get_input("interval")?;
			let interval = Duration::from_millis(input);
			let children_count = bhvr_.children.len();

			// @TODO: Dirty way to move access to children into spawned task
			//        The node is not restartable/recoverable
			let mut children: Vec<Behavior> = Vec::new();
			std::mem::swap(&mut bhvr_.children, &mut children);

			self.handle
				.replace(tokio::task::spawn(async move {
					let mut interval = time::interval(interval);
					loop {
						interval.tick().await;

						// tick every child
						for mut child in &mut children {
							child.execute_tick().await;
						}
					}
				}));
		}

		Ok(bhvr_.status)
		// Ok(BehaviorStatus::Running)
	}

	async fn halt(&self) {
		bhvr_.reset_children().await;
		let handle = self.handle.take();
		if let Some(handle) = handle {
			handle.abort();
		};
		// @TODO: clarify which status is best
		bhvr_.set_status(BehaviorStatus::Success);
	}

	fn ports() -> PortList {
		// input parameter "interval" with default of 1000ms
		define_ports!(input_port!("interval", 1000))
	}

	/// Registration function
	pub fn register(factory: &mut BTFactory) {
		register_decorator!(factory, "IntervalTimer", IntervalTimer,);
	}
}
// endregion:   --- behavior
