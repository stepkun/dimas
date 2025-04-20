// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

//! A library with test behaviors

use alloc::sync::Arc;
use dimas_behavior::{
	factory::{BehaviorRegistry, NewBehaviorTreeFactory},
	input_port_macro,
	new_behavior::{BehaviorCreationMethods, NewBehaviorType, SimpleBehavior}, port_list,
};
use parking_lot::Mutex;

use crate::test_nodes::{
	ApproachObject, CalculateGoal, GripperInterface, MoveBaseAction, PrintTarget, SaySomething,
	ThinkWhatToSay, check_battery, say_something_simple,
};

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(factory: &mut NewBehaviorTreeFactory) -> u32 {
	// t01
	factory
		.register_simple_condition("CheckBattery", Arc::new(check_battery))
		.unwrap();
	factory
		.register_node_type::<ApproachObject>("ApproachObject")
		.unwrap();
	let gripper1 = Arc::new(Mutex::new(GripperInterface::default()));
	let gripper2 = gripper1.clone();
	// @TODO: replace the workaround with a solution!
	factory
		.register_simple_action("OpenGripper", Arc::new(move || gripper1.lock().open()))
		.unwrap();
	factory
		.register_simple_action("CloseGripper", Arc::new(move || gripper2.lock().close()))
		.unwrap();

	// t02
	factory
		.register_node_type::<SaySomething>("SaySomething")
		.unwrap();
	factory
		.register_node_type::<ThinkWhatToSay>("ThinkWhatToSay")
		.unwrap();
	// [`SimpleBehavior`]s can not define their own method provided_ports(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	let say_something_ports = port_list![input_port_macro!(String, "message")];
	factory
		.register_simple_action_with_ports(
			"SaySomething2",
			Arc::new(say_something_simple),
			say_something_ports,
		)
		.unwrap();

	// t03
	factory
		.register_node_type::<CalculateGoal>("CalculateGoal")
		.unwrap();
	factory
		.register_node_type::<PrintTarget>("PrintTarget")
		.unwrap();

	// t04
	factory
		.register_simple_condition("BatteryOK", Arc::new(check_battery))
		.unwrap();
	factory
		.register_node_type::<MoveBaseAction>("MoveBase")
		.unwrap();

	// A return value of 0 signals success
	0
}
