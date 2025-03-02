// Copyright © 2024 Stephan Kunz

//! This test implements the eigth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_08_additional_args)
//!

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus},
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <ActionA message="Running ActionA" />
            <ActionB message="Running ActionB" />
            <ActionC message="Running ActionC" />
        </Sequence>
    </BehaviorTree>

	<!-- Description of Node Models (used by Groot) -->
	<TreeNodesModel>
		<Action ID="ActionA"
				editable="true"/>
		<Action ID="ActionB"
				editable="true"/>
		<Action ID="ActionC"
				editable="true"/>
	</TreeNodesModel>
</root>
"#;

/// SyncAction "ActionA"
#[behavior(SyncAction)]
struct ActionA {
	arg1: i32,
	arg2: String,
}

#[behavior(SyncAction)]
impl ActionA {
	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config_mut().get_input("message")?;

		let arg1 = self.arg1;
		let arg2 = &self.arg2;

		assert_eq!(arg1, 42);
		assert_eq!(arg2, "hello world");

		println!("{msg} robot says: {arg2}, the answer is {arg1}!");

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("message"))
	}
}

/// SyncAction "ActionB"
#[behavior(SyncAction)]
struct ActionB {
	#[bhvr(default)]
	arg1: i32,
	#[bhvr(default)]
	arg2: String,
}

#[behavior(SyncAction)]
impl ActionB {
	fn ports() -> PortList {
		define_ports!(input_port!("message"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config_mut().get_input("message")?;

		// println!("{msg} is currently not implementable");
		let arg1 = self.arg1;
		let arg2 = &self.arg2;

		assert_eq!(arg1, 42);
		assert_eq!(arg2, "hello world");

		println!("{msg} robot says: {arg2}, the answer is {arg1}!");

		Ok(BehaviorStatus::Success)
	}

	fn initialize(&mut self, arg1: i32, arg2: String) {
		self.arg1 = arg1;
		self.arg2 = arg2;
	}
}

/// SyncAction "ActionC"
#[behavior(SyncAction)]
struct ActionC {
	#[bhvr(default = "42")]
	arg1: i32,
	#[bhvr(default = "String::from(\"hello world\")")]
	arg2: String,
}

#[behavior(SyncAction)]
impl ActionC {
	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config_mut().get_input("message")?;

		let arg1 = self.arg1;
		let arg2 = &self.arg2;

		assert_eq!(arg1, 42);
		assert_eq!(arg2, "hello world");

		println!("{msg} robot says: {arg2}, the answer is {arg1}!");

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("message"))
	}
}

#[tokio::test]
async fn additional_args() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	let arg1 = 42;
	let arg2 = String::from("hello world");

	// registering with a different constructor
	register_action!(factory, "ActionA", ActionA, arg1, arg2);
	// registering for using initialize function
	register_action!(factory, "ActionB", ActionB);
	// registering using defaults
	register_action!(factory, "ActionC", ActionC);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// initialize ActionB with the help of an iterator
	for node in tree.iter_mut() {
		if node.name() == "ActionB" {
			let action = node
				.context_mut()
				.downcast_mut::<ActionB>()
				.expect("snh");
			action.initialize(42, "hello world".into());
		}
	}

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	// test the iterator
	let mut iter = tree.iter();
	assert_eq!(iter.next().expect("snh").name(), "Sequence");
	assert_eq!(iter.next().expect("snh").name(), "ActionA");
	assert_eq!(iter.next().expect("snh").name(), "ActionB");
	assert_eq!(iter.next().expect("snh").name(), "ActionC");
	assert!(iter.next().is_none());

	Ok(())
}
