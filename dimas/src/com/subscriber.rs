// Copyright © 2025 Stephan Kunz

//! Subscriber

// region:      --- modules
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorData, BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
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
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("starting Subscriber");

		self.tick(behavior, children, runtime).await
	}

	/// @TODO:
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorState::Running)
	}
}

impl BehaviorStatic for Subscriber {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
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
