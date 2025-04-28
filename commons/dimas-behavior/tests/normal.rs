// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Tests

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	//is_normal::<Behavior>();
	// is_normal::<BehaviorFunction>();
	// is_normal::<BehaviorTree>();
	// is_normal::<Blackboard>();
	// is_normal::<Port>();
}
