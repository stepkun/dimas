// Copyright © 2025 Stephan Kunz

//! Interval timer

// region:      --- modules
use core::time::Duration;
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType, error::BehaviorError},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
use tokio::{task::JoinHandle, time};
// endregion:   --- modules

// region:      --- IntervalTimer
/// An [`IntervalTimer`]
#[derive(Behavior, Debug, Default)]
pub struct IntervalTimer {
	/// The handle to stop the Timer
	handle: Option<JoinHandle<()>>,
}

#[async_trait::async_trait]
impl BehaviorInstance for IntervalTimer {
	async fn start(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// timer already started?
		if self.handle.is_none() {
			let input = blackboard.get("interval".into())?;
			let interval = Duration::from_millis(input);
			let _children_count = children.len();

			// @TODO: Dirty way to move access to children into spawned task
			//        The node is not properly restartable/recoverable
			let mut my_children: BehaviorTreeElementList = BehaviorTreeElementList::default();
			std::mem::swap(children, &mut my_children);

			let runtime = runtime.clone();
			self.handle
				.replace(tokio::task::spawn(async move {
					let mut interval = time::interval(interval);
					loop {
						interval.tick().await;

						// tick every child
						for index in 0..my_children.len() {
							let child = &mut my_children[index];
							let _new_state = child.execute_tick(&runtime).await;
						}
					}
				}));
			Ok(BehaviorState::Running)
		} else {
			println!("already started IntervalTimer");
			Ok(BehaviorState::Failure)
		}
	}

	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		Ok(BehaviorState::Running)
	}

	async fn halt(
		&mut self,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		children.reset(runtime)?;
		if let Some(handle) = self.handle.take() {
			// @TODO: Dirty way to move access to children back from spawned task
			//        The node is not properly restartable/recoverable
			let mut my_children: BehaviorTreeElementList = BehaviorTreeElementList::default();
			std::mem::swap(children, &mut my_children);
			children.reset(runtime)?;

			handle.abort();
		}
		Ok(())
	}
}

impl BehaviorStatic for IntervalTimer {
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			i32,
			"interval",
			"1000",
			"Default value in ms."
		)]
	}
}
// endregion:   --- IntervalTimer
