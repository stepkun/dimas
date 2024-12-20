// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]
#![allow(missing_docs)]

//! Subscriber

// region:      --- modules
use behaviortree_rs::{
	bt_node, macros::register_action_node, basic_types::NodeStatus, Factory, nodes::NodeResult,
};
// endregion:   --- modules

// region:      --- behavior
/// ActionNode "Subscriber"
#[bt_node(StatefulActionNode)]
pub struct Subscriber {}

#[allow(clippy::use_self)]
#[bt_node(StatefulActionNode)]
impl Subscriber {
	async fn on_start(&mut self) -> NodeResult {
		println!("starting Subscriber");
		Ok(NodeStatus::Running)
	}

	async fn on_running(&mut self) -> NodeResult {
		println!("ticking Subscriber");
		Ok(NodeStatus::Running)
	}

	/// Registration function
	pub fn register(bt_factory: &mut Factory) {
		register_action_node!(bt_factory, "Subscriber", Subscriber);
	}
}
// endregion:   --- behavior
