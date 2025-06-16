// Copyright © 2025 Stephan Kunz

//! A library with crosss door behaviors

use dimas_behavior::factory::BehaviorTreeFactory;

use crate::cross_door::CrossDoor;

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(factory: &mut BehaviorTreeFactory) -> u32 {
	let cross_door = CrossDoor::default();
	cross_door
		.register_behaviors(factory)
		.expect("snh");
	0
}
