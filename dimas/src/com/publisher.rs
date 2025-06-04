// Copyright © 2025 Stephan Kunz

//! Publisher

// region:      --- modules
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
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

#[async_trait::async_trait]
impl BehaviorInstance for Publisher {
	/// @TODO:
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ticking Publisher");
		Ok(BehaviorState::Success)
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
