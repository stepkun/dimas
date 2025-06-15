// Copyright © 2025 Stephan Kunz

//! Subscriber

// region:      --- modules
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorData, BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
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
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("starting Subscriber");

		self.tick(behavior, blackboard, children, runtime)
			.await
	}

	/// @TODO:
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorState::Running)
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
