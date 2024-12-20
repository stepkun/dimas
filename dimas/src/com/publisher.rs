// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]
#![allow(missing_docs)]

//! Publisher

// region:      --- modules
use behaviortree_rs::{
	bt_node, macros::register_action_node, basic_types::NodeStatus, Factory, nodes::NodeResult,
};
// endregion:   --- modules

// region:      --- behavior
/// ActionNode "Publisher"
#[bt_node(SyncActionNode)]
pub struct Publisher {}

#[allow(clippy::use_self)]
#[bt_node(SyncActionNode)]
impl Publisher {
	async fn tick(&mut self) -> NodeResult {
		println!("ticking Publisher");
		Ok(NodeStatus::Success)
	}

	/// Registration function
	pub fn register(bt_factory: &mut Factory) {
		register_action_node!(bt_factory, "Publisher", Publisher);
	}
}
// endregion:   --- behavior
