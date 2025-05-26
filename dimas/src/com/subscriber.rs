// Copyright © 2025 Stephan Kunz

//! Subscriber

// region:      --- modules
use dimas_behavior::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorType},
	blackboard::SharedBlackboard,
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Subscriber
/// A [`Subscriber`]
#[derive(Behavior, Debug, Default)]
pub struct Subscriber {}

impl BehaviorInstance for Subscriber {
	/// @TODO:
	fn tick(
		&mut self,
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for Subscriber {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"topic",
			"",
			"Topic to subscribe."
		)]
	}
}
// endregion:   --- Subscriber
