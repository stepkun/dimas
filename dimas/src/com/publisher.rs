// Copyright © 2025 Stephan Kunz

//! Publisher

// region:      --- modules
use dimas_behavior::{
	behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, BehaviorType,
	},
	blackboard::BlackboardNodeRef,
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

impl BehaviorInstanceMethods for Publisher {
	/// @TODO:
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut BlackboardNodeRef,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		println!("ticking Publisher");
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for Publisher {
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
