// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! A library with test behaviors

use alloc::sync::Arc;
use dimas_behavior::{
	factory::BehaviorRegistry,
	new_behavior::{BehaviorCreationMethods, NewBehaviorType, SimpleBehavior},
	new_port::input_port,
};

use crate::test_nodes::{
	ApproachObject, CalculateGoal, GripperInterface, MoveBaseAction, PrintTarget, SaySomething,
	ThinkWhatToSay, check_battery, say_something_simple,
};

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(registry: &mut BehaviorRegistry) -> u32 {
	registry.register_behavior(
		"CheckBattery",
		SimpleBehavior::create(Arc::new(check_battery)),
		NewBehaviorType::Condition,
	);

	registry.register_behavior(
		"ApproachObject",
		ApproachObject::create(),
		NewBehaviorType::Action,
	);

	let gripper1 = Arc::new(GripperInterface::default());
	let gripper2 = gripper1.clone();
	registry.register_behavior(
		"OpenGripper",
		SimpleBehavior::create(Arc::new(move || gripper1.open())),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"CloseGripper",
		SimpleBehavior::create(Arc::new(move || gripper2.close())),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"SaySomething",
		SaySomething::create(),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"ThinkWhatToSay",
		ThinkWhatToSay::create(),
		NewBehaviorType::Action,
	);

	// [`SimpleBehavior`]s can not define their own method provided_ports(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	let mut say_something_ports = vec![input_port::<String>("message", "", "").expect("snh")];
	registry.register_behavior(
		"SaySomething2",
		SimpleBehavior::create_with_ports(Arc::new(say_something_simple), say_something_ports),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"CalculateGoal",
		CalculateGoal::create(),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"PrintTarget",
		PrintTarget::create(),
		NewBehaviorType::Action,
	);

	registry.register_behavior(
		"BatteryOK",
		SimpleBehavior::create(Arc::new(check_battery)),
		NewBehaviorType::Condition,
	);
	registry.register_behavior(
		"MoveBase",
		MoveBaseAction::create(),
		NewBehaviorType::Action,
	);

	// A return value of 0 signals success
	0
}
