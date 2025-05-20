// Copyright © 2025 Stephan Kunz

//! Publisher

// region:      --- modules
use dimas_behavior::{
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	input_port_macro,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
};
use dimas_behavior_derive::Behavior;
// endregion:   --- modules

// region:      --- Publisher
/// A [`Publisher`]
#[derive(Behavior, Debug, Default)]
pub struct Publisher {}

impl BehaviorInstance for Publisher {
	/// @TODO:
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		println!("ticking Publisher");
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for Publisher {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port_macro!(
			String,
			"topic",
			"",
			"Topic to publish."
		)]
	}
}
// endregion:   --- Publisher
