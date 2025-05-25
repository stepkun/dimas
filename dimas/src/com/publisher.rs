// Copyright © 2025 Stephan Kunz

//! Publisher

// region:      --- modules
use dimas_behavior::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
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
		_children: &mut BehaviorTreeElementList,
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
		port_list![input_port!(
			String,
			"topic",
			"",
			"Topic to publish."
		)]
	}
}
// endregion:   --- Publisher
