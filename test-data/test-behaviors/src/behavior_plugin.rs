// Copyright © 2025 Stephan Kunz

//! A library with test behaviors

use dimas_behavior::{behavior::BehaviorType, factory::BehaviorTreeFactory, input_port, port_list, register_behavior};

use crate::test_nodes::{
	ApproachObject, CalculateGoal, GripperInterface, MoveBaseAction, PrintTarget, SaySomething, ThinkWhatToSay,
	check_battery, say_something_simple,
};

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(factory: &mut BehaviorTreeFactory) -> u32 {
	// t01
	register_behavior!(factory, check_battery, "CheckBattery", BehaviorType::Condition).expect("snh");
	register_behavior!(factory, ApproachObject, "ApproachObject").expect("snh");
	register_behavior!(
		factory,
		GripperInterface::default(),
		open,
		"OpenGripper",
		BehaviorType::Action,
		close,
		"CloseGripper",
		BehaviorType::Action,
	)
	.expect("snh");

	// t02
	register_behavior!(factory, SaySomething, "SaySomething").expect("snh");
	register_behavior!(factory, ThinkWhatToSay, "ThinkWhatToSay").expect("snh");
	// [`SimpleBehavior`]s can not define their own method provided_ports(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	let say_something_ports = port_list![input_port!(String, "message")];
	register_behavior!(
		factory,
		say_something_simple,
		"SaySomething2",
		say_something_ports,
		BehaviorType::Action
	)
	.expect("snh");

	// t03
	register_behavior!(factory, CalculateGoal, "CalculateGoal").expect("snh");
	register_behavior!(factory, PrintTarget, "PrintTarget").expect("snh");

	// t04
	register_behavior!(factory, check_battery, "BatteryOK", BehaviorType::Condition).expect("snh");
	register_behavior!(factory, MoveBaseAction, "MoveBase").expect("snh");

	// A return value of 0 signals success
	0
}
