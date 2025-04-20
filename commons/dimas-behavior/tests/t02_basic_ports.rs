// Copyright © 2025 Stephan Kunz

//! This test implements the second tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_02_basic_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t02_basic_ports.cpp)
//!

#[doc(hidden)]
extern crate alloc;

use std::sync::Arc;

use dimas_behavior::{
	factory::NewBehaviorTreeFactory, input_port_macro, new_behavior::NewBehaviorStatus, port_list,
};
use serial_test::serial;
use test_behaviors::test_nodes::{SaySomething, ThinkWhatToSay, say_something_simple};

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root">
			<SaySomething     message="hello" />
			<SaySomething2    message="this works too" />
			<ThinkWhatToSay   text="{the_answer}"/>
			<SaySomething     message="{the_answer}" />
			<SaySomething2    message="{the_answer}" />
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
async fn basic_ports() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	// The struct SaySomething has a method called ports() that defines the INPUTS.
	// In this case, it requires an input called "message"
	factory.register_node_type::<SaySomething>("SaySomething")?;

	// Similarly to SaySomething, ThinkWhatToSay has an OUTPUT port called "text"
	// Both these ports are of type `String`, therefore they can connect to each other
	factory.register_node_type::<ThinkWhatToSay>("ThinkWhatToSay")?;

	// [`SimpleBehavior`]s can not define their own method provided_ports(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	let say_something_ports = port_list!{input_port_macro!(String, "message")};
	factory.register_simple_action_with_ports(
		"SaySomething2",
		Arc::new(say_something_simple),
		say_something_ports,
	)?;

	let mut tree = factory.create_from_text(XML)?;

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
#[serial]
async fn basic_ports_with_plugin() -> anyhow::Result<()> {
	extern crate std;
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("libtest_behaviors")?;

	let mut tree = factory.create_from_text(XML)?;

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}
