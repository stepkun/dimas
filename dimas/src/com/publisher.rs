// Copyright © 2025 Stephan Kunz

//! Publisher

// region:      --- modules
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorData, BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
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
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("starting Publisher");

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
		println!("ticking Publisher");
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for Publisher {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![
			input_port!(String, "topic", "", "Topic to publish."),
			input_port!(String, "message", "", "Message to publish.")
		]
	}
}
// endregion:   --- Publisher
