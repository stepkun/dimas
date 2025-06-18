// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Tests for [`Blackboard`] and [`BlackboardNode`]

use dimas_behavior::{
	blackboard::{BlackboardData, BlackboardInterface, SharedBlackboard},
	port::PortRemappings,
};

#[test]
fn blackboard() {
	let mut blackboard = BlackboardData::default();

	let value = blackboard.get::<i32>("test");
	assert!(value.is_err());

	let old = blackboard
		.set("test", String::from("test"))
		.expect("snh");
	assert_eq!(old, None);

	let value = blackboard.get::<String>("test").expect("snh");
	assert_eq!(value, String::from("test"));

	let old = blackboard
		.set("test", String::from("changed"))
		.expect("snh");
	assert_eq!(old, Some(String::from("test")));

	let value = blackboard.get::<String>("test").expect("snh");
	assert_eq!(value, String::from("changed"));

	let value = blackboard.set("test", 42);
	assert!(value.is_err());

	let old = blackboard.delete::<String>("test").expect("snh");
	assert_eq!(old, String::from("changed"));

	let old = blackboard.set("test", 42).expect("snh");
	assert_eq!(old, None);

	let value = blackboard.get::<i32>("test").expect("snh");
	assert_eq!(value, 42);
}

#[test]
fn blackboard_node_default() {
	let remappings0 = PortRemappings::default();
	let values0 = PortRemappings::default();
	let mut level0 = SharedBlackboard::new("level0", remappings0, values0);

	let value = level0.get::<i32>("test");
	assert!(value.is_err());

	let old = level0
		.set("test", String::from("test"))
		.expect("snh");
	assert_eq!(old, None);

	let value = level0.get::<String>("test").expect("snh");
	assert_eq!(value, String::from("test"));

	let old = level0.set("test", 42);
	assert!(old.is_err());

	let old = level0.delete::<String>("test").expect("snh");
	assert_eq!(old, String::from("test"));

	let old = level0.set("test", 42).expect("snh");
	assert_eq!(old, None);

	let value = level0.get::<i32>("test").expect("snh");
	assert_eq!(value, 42);
}

#[test]
fn blackboard_node_with_parent() {
	let remappings0 = PortRemappings::default();
	let values0 = PortRemappings::default();
	let mut level0 = SharedBlackboard::new("level0", remappings0, values0);

	let old = level0
		.set("test1", String::from("test1"))
		.expect("snh");
	assert_eq!(old, None);
	let old = level0
		.set("test2", String::from("test2"))
		.expect("snh");
	assert_eq!(old, None);

	let mut remappings = PortRemappings::default();
	let values = PortRemappings::default();
	remappings.add("test", "test1");
	let mut node = SharedBlackboard::with("level0", level0, remappings, values, true);

	let old = node
		.set("@other", String::from("other"))
		.expect("snh");
	assert_eq!(old, None);

	let old = node
		.set("test3", String::from("test3"))
		.expect("snh");
	assert_eq!(old, None);

	let value = node.get::<String>("@other").expect("snh");
	assert_eq!(value, String::from("other"));
	let value = node.get::<String>("test").expect("snh");
	assert_eq!(value, String::from("test1"));
	let value = node.get::<String>("test2").expect("snh");
	assert_eq!(value, String::from("test2"));
	let value = node.get::<String>("test3").expect("snh");
	assert_eq!(value, String::from("test3"));
}

#[test]
fn blackboard_node_hierarchy() {
	let remappings0 = PortRemappings::default();
	let values0 = PortRemappings::default();
	let mut level0 = SharedBlackboard::new("level0", remappings0, values0);

	let mut remappings1 = PortRemappings::default();
	let values1 = PortRemappings::default();

	remappings1.add("levelB", "levelA");
	let mut level1 = SharedBlackboard::with("level1", level0.clone(), remappings1, values1, true);

	let mut remappings2 = PortRemappings::default();
	let values2 = PortRemappings::default();

	remappings2.add("levelC", "levelB");
	let mut level2 = SharedBlackboard::with("level2", level1.clone(), remappings2, values2, true);

	let mut remappings3 = PortRemappings::default();
	let values3 = PortRemappings::default();
	remappings3.add("levelD", "levelC");
	let mut level3 = SharedBlackboard::with("level3", level2.clone(), remappings3, values3, true);

	let old = level0
		.set("levelA", String::from("testA"))
		.expect("snh");
	assert_eq!(old, None);

	let old = level0
		.set("level0", String::from("test0"))
		.expect("snh");
	assert_eq!(old, None);
	let old = level1
		.set("level1", String::from("test1"))
		.expect("snh");
	assert_eq!(old, None);
	let old = level2
		.set("level2", String::from("test2"))
		.expect("snh");
	assert_eq!(old, None);
	let old = level3
		.set("level3", String::from("test3"))
		.expect("snh");
	assert_eq!(old, None);

	// test autoremap
	let old = level3
		.set("level2", String::from("changed2"))
		.expect("snh");
	assert_eq!(old, Some(String::from("test2")));
	let value = level3.get::<String>("level2").expect("snh");
	assert_eq!(value, String::from("changed2"));
	let value = level3.get::<String>("level1").expect("snh");
	assert_eq!(value, String::from("test1"));
	let value = level3.get::<String>("level0").expect("snh");
	assert_eq!(value, String::from("test0"));
	let old = level3
		.set("level0", String::from("changed0"))
		.expect("snh");
	assert_eq!(old, Some(String::from("test0")));
	let value = level3.get::<String>("level0").expect("snh");
	assert_eq!(value, String::from("changed0"));
	let old = level3.set("level0", 42);
	assert!(old.is_err());
	let old = level3.delete::<String>("level0").expect("snh");
	assert_eq!(value, String::from("changed0"));
	let old = level3.set("level0", 42).expect("snh");
	assert_eq!(old, None);
	let value = level1.get::<i32>("level0").expect("snh");
	assert_eq!(value, 42);

	// test manual remapping
	let value = level3.get::<String>("levelD").expect("snh");
	assert_eq!(value, String::from("testA"));
	let value = level2.get::<String>("levelC").expect("snh");
	assert_eq!(value, String::from("testA"));
	let old = level3
		.set("levelD", String::from("changedD"))
		.expect("snh");
	assert_eq!(old, Some(String::from("testA")));
	let value = level1.get::<String>("levelB").expect("snh");
	assert_eq!(value, String::from("changedD"));
	let old = level3.set("levelD", 42);
	assert!(old.is_err());
	let old = level3.delete::<String>("levelD").expect("snh");
	assert_eq!(value, String::from("changedD"));
	let old = level3.set("levelD", 42).expect("snh");
	assert_eq!(old, None);
	let value = level1.get::<i32>("levelB").expect("snh");
	assert_eq!(value, 42);
}
