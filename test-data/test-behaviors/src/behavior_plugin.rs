// Copyright © 2025 Stephan Kunz

//! A library with test behaviors

use alloc::sync::Arc;
use dimas_behavior::{factory::BehaviorTreeFactory, input_port, port_list};
use parking_lot::Mutex;

use crate::test_nodes::{
	AlwaysFailure, AlwaysSuccess, ApproachObject, CalculateGoal, GripperInterface, MoveBaseAction,
	PrintTarget, SaySomething, ThinkWhatToSay, check_battery, new_say_something_simple,
};

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(factory: &mut BehaviorTreeFactory) -> u32 {
	// t01
	factory
		.register_simple_condition("CheckBattery", Arc::new(check_battery))
		.expect("snh");
	factory
		.register_node_type::<ApproachObject>("ApproachObject")
		.expect("snh");
	let gripper1 = Arc::new(Mutex::new(GripperInterface::default()));
	let gripper2 = gripper1.clone();
	// @TODO: replace the workaround with a solution!
	factory
		.register_simple_action("OpenGripper", Arc::new(move || gripper1.lock().open()))
		.expect("snh");
	factory
		.register_simple_action("CloseGripper", Arc::new(move || gripper2.lock().close()))
		.expect("snh");

	// t02
	factory
		.register_node_type::<SaySomething>("SaySomething")
		.expect("snh");
	factory
		.register_node_type::<ThinkWhatToSay>("ThinkWhatToSay")
		.expect("snh");
	// [`SimpleBehavior`]s can not define their own method provided_ports(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	let say_something_ports = port_list![input_port!(String, "message")];
	factory
		.register_simple_action_with_ports(
			"SaySomething2",
			Arc::new(new_say_something_simple),
			say_something_ports,
		)
		.expect("snh");

	// t03
	factory
		.register_node_type::<CalculateGoal>("CalculateGoal")
		.expect("snh");
	factory
		.register_node_type::<PrintTarget>("PrintTarget")
		.expect("snh");

	// t04
	factory
		.register_simple_condition("BatteryOK", Arc::new(check_battery))
		.expect("snh");
	factory
		.register_node_type::<MoveBaseAction>("MoveBase")
		.expect("snh");

	// t10
	factory
		.register_node_type::<AlwaysFailure>("AlwaysFailure")
		.expect("snh");
	factory
		.register_node_type::<AlwaysSuccess>("AlwaysSuccess")
		.expect("snh");

	// A return value of 0 signals success
	0
}
