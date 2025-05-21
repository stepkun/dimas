// Copyright © 2025 Stephan Kunz

//! Interval timer

// region:      --- modules
use core::time::Duration;
use dimas_behavior::{
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType, error::BehaviorError,
	},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port,
	port::PortList,
	port_list,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentList},
};
use dimas_behavior_derive::Behavior;
use tokio::{task::JoinHandle, time};
// endregion:   --- modules

// region:      --- IntervalTimer
/// An [`IntervalTimer`]
#[derive(Behavior, Debug, Default)]
pub struct IntervalTimer {
	/// The handle to stop the Timer
	handle: Option<JoinHandle<()>>,
}

impl BehaviorInstance for IntervalTimer {
	fn start(
		&mut self,
		tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		println!("start IntervalTimer");

		// timer already started?
		if self.handle.is_none() {
			tick_data.set_status(BehaviorStatus::Running);

			let input = blackboard.get("interval".into())?;
			let interval = Duration::from_millis(input);
			let _children_count = children.len();

			// @TODO: Dirty way to move access to children into spawned task
			//        The node is not restartable/recoverable
			let mut my_children: BehaviorTreeComponentList = BehaviorTreeComponentList::default();
			std::mem::swap(children, &mut my_children);

			self.handle
				.replace(tokio::task::spawn(async move {
					let mut interval = time::interval(interval);
					loop {
						interval.tick().await;

						// tick every child
						for index in 0..my_children.len() {
							let child = &mut my_children[index];
							let _new_status = child.execute_tick();
						}
					}
				}));
		} else {
			println!("already started IntervalTimer");
			tick_data.set_status(BehaviorStatus::Failure);
		}

		Ok(tick_data.status())
		// Ok(BehaviorStatus::Running)
	}

	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		println!("ticking IntervalTimer");
		Ok(BehaviorStatus::Running)
	}

	fn halt(&mut self, children: &mut BehaviorTreeComponentList) -> Result<(), BehaviorError> {
		children.reset()?;
		let handle = self.handle.take();
		if let Some(handle) = handle {
			handle.abort();
		}
		Ok(())
	}
}

impl BehaviorStatic for IntervalTimer {
	fn kind() -> BehaviorType {
		BehaviorType::Action
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
