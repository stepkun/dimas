// Copyright © 2025 Stephan Kunz

//! Subscriber

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

// region:      --- Subscriber
/// A [`Subscriber`]
#[derive(Behavior, Debug, Default)]
pub struct Subscriber {}

#[async_trait::async_trait]
impl BehaviorInstance for Subscriber {
	/// @TODO:
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorState::Success)
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
