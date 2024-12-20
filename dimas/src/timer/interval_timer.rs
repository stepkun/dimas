// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]
#![allow(clippy::use_self)]
#![allow(clippy::unwrap_used)]
#![allow(missing_docs)]

//! Interval timer

// region:      --- modules
use anyhow::Result;
use behaviortree_rs::{
	basic_types::{NodeCategory, NodeStatus}, bt_node, macros::{define_ports, input_port, register_decorator_node}, nodes::{NodeResult, PortsList, TreeNode}, Factory
};
use tokio::{task::JoinHandle, time};
use std::time::Duration;
// endregion:   --- modules

// region:      --- behavior
/// An [`IntervalTimer`]
#[bt_node(DecoratorNode)]
pub struct IntervalTimer {
	/// The handle to stop the Timer
	#[bt(default = "None")]
	handle: Option<JoinHandle<()>>,

}

#[bt_node(DecoratorNode)]
impl IntervalTimer {
	async fn tick(&mut self) -> NodeResult {
		println!("ticking IntervalTimer");

		// timer already started?
		if self.handle.is_none() {
			node_.set_status(NodeStatus::Running);

			let input = node_.config.get_input("interval")?;
			let interval = Duration::from_millis(input);
			let children_count = node_.children.len();

			// @TODO: Dirty way to move access to children into spawned task
			//        The node is not restartable/recoverable
			let mut children: Vec<TreeNode> = Vec::new();
			std::mem::swap(&mut node_.children, &mut children);
			
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

		Ok(node_.status)
	}

	async fn halt(&mut self) {
		node_.reset_children().await;
		let handle = self.handle.take();
		if let Some(handle) = handle {
			handle.abort();
		};
		// @TODO: clarify which status is best
		node_.set_status(NodeStatus::Success);
	}

	fn ports() -> PortsList {
		// input parameter "interval" with default of 1000ms
        define_ports!(input_port!("interval", 1000))
	}

	/// Registration function
	pub fn register(bt_factory: &mut Factory) {
		register_decorator_node!(bt_factory, "IntervalTimer", IntervalTimer);
	}
}
// endregion:   --- behavior
