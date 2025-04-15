// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! This test implements the second tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_02_basic_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t02_basic_ports.cpp)
//!

#[doc(hidden)]
extern crate alloc;

mod test_nodes;

use std::sync::Arc;

use dimas_behavior::{
	factory::NewBehaviorTreeFactory,
	new_behavior::{BehaviorResult, NewBehaviorStatus},
	port::PortList,
};
use test_nodes::{ApproachObject, GripperInterface, SaySomething, ThinkWhatToSay, check_battery};

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root">
			<SaySomething     message="hello" />
			<SaySomething2    message="this works too" />
			<ThinkWhatToSay   text="{the_answer}"/>
			<SaySomething2    message="{the_answer}" />
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn basic_ports() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::default();

	// The struct SaySomething has a method called ports() that defines the INPUTS.
	// In this case, it requires an input called "message"
	factory.register_node_type::<SaySomething>("SaySomething");

	// Similarly to SaySomething, ThinkWhatToSay has an OUTPUT port called "text"
	// Both these ports are of type `String`, therefore they can connect to each other
	factory.register_node_type::<ThinkWhatToSay>("ThinkWhatToSayg");

	// SimpleActionNodes can not define their own method providedPorts(), therefore
	// we have to pass the PortsList explicitly if we want the Action to use get_input()
	// or set_output();
	// let mut say_something_ports = PortList::from({ InputPort::<String>::new("message") });
	// factory.register_simple_action("SaySomething2", SaySomethingSimple, say_something_ports);

	let mut tree = factory.create_from_text(XML)?;
	//dbg!(&tree);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}
