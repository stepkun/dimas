// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! A library with crosss door behaviors

use dimas_behavior::factory::NewBehaviorTreeFactory;

use crate::cross_door::CrossDoor;

/// Registration function for all external symbols
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
extern "Rust" fn register(factory: &mut NewBehaviorTreeFactory) -> u32 {
	let cross_door = CrossDoor::default();
	cross_door.register_nodes(factory).unwrap();
	0
}
