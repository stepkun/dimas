// Copyright © 2025 Stephan Kunz

//! Subscriber

// region:      --- modules
use dimas_behavior::{
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
};
use dimas_behavior_derive::Behavior;
// endregion:   --- modules

// region:      --- Subscriber
/// A [`Subscriber`]
#[derive(Behavior, Debug, Default)]
pub struct Subscriber {}

impl BehaviorInstance for Subscriber {
	/// @TODO:
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
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
