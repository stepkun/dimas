// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_self)]
#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]
#![allow(unused)]

//! Test behaviors
//!

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	define_ports, input_port,
	new_behavior::{
		BehaviorCreation, BehaviorCreationFn, BehaviorMethods, BehaviorResult, BehaviorTickData,
		NewBehaviorStatus, NewBehaviorType,
	},
	output_port,
	port::PortList,
	tree::BehaviorTreeComponent,
};

/// Behavior `ApproachObject`
/// Example of custom `SyncActionNode` (synchronous action) without ports.
#[derive(Debug)]
pub struct ApproachObject {}

impl BehaviorCreation for ApproachObject {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorMethods for ApproachObject {
	fn tick(&self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		println!("ApproachObject: approach_object");
		Ok(NewBehaviorStatus::Success)
	}
}

/// Function for behavior `CheckBattery`
/// # Errors
/// In this case never :-)
pub fn check_battery() -> BehaviorResult {
	println!("[ Battery: OK ]");
	Ok(NewBehaviorStatus::Success)
}

/// Struct for behaviors `OpenGripper` and `CloseGripper`
#[derive(Default)]
pub struct GripperInterface {}

impl GripperInterface {
	/// Open the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn open(&self) -> BehaviorResult {
		println!("GripperInterface::open");
		Ok(NewBehaviorStatus::Success)
	}
	/// Close the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn close(&self) -> BehaviorResult {
		println!("GripperInterface::close");
		Ok(NewBehaviorStatus::Success)
	}
}
/// Behavior `SaySomething`
/// Example of custom `SyncActionNode` (synchronous action) with an input port.
#[derive(Debug)]
pub struct SaySomething {}

impl BehaviorCreation for SaySomething {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorMethods for SaySomething {
	// fn ports(&self) -> PortList {
	// 	define_ports!(input_port!("message", "hello"))
	// }

	fn tick(&self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let msg = tree_node
			.tick_data
			.lock()
			.get_input::<String>("message")?;
		Ok(NewBehaviorStatus::Success)
	}
}

// @TODO: make it work
// /// Same as struct SaySomething, but to be registered with SimpleActionNode
// fn say_something_simple(BT::TreeNode& self) -> BehaviorResult;

/// Behavior `ThinkWhatToSay`
#[derive(Debug)]
pub struct ThinkWhatToSay {}

impl BehaviorCreation for ThinkWhatToSay {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorMethods for ThinkWhatToSay {
	// fn ports(&self) -> PortList {
	// 	define_ports!(output_port!("text"))
	// }

	fn tick(&self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		tree_node
			.tick_data
			.lock()
			.set_output("text", "The answer is 42.")?;
		Ok(NewBehaviorStatus::Success)
	}
}
