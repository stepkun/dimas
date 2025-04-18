// Copyright © 2025 Stephan Kunz

//! This test implements the first tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_01_first_tree)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t01_build_your_first_tree.cpp)
//!

use std::sync::Arc;

use dimas_behavior::{factory::NewBehaviorTreeFactory, new_behavior::NewBehaviorStatus};
use test_behaviors::test_nodes::{ApproachObject, GripperInterface, check_battery};

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root_sequence">
			<CheckBattery	name="battery_ok"/>
			<OpenGripper	name="open_gripper"/>
			<ApproachObject	name="approach_object"/>
			<CloseGripper	name="close_gripper"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn build_your_first_tree() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	// The recommended way to create a Behavior is through inheritance/composition.
	// Even if it requires more boilerplate, it allows you to use more functionalities
	// like ports (we will discuss this in future tutorials).
	factory.register_node_type::<ApproachObject>("ApproachObject")?;

	// Registering a SimpleAction/SimpleCondition using a function pointer.
	factory.register_simple_condition("CheckBattery", Arc::new(check_battery))?;

	// You can also create SimpleAction/SimpleCondition using methods of a struct.
	// In Rust this needs to be done with Closures and an Arc to the struct.
	let gripper1 = Arc::new(GripperInterface::default());
	let gripper2 = gripper1.clone();
	// @TODO: replace the workaround with a solution!
	factory.register_simple_action("OpenGripper", Arc::new(move || gripper1.open()))?;
	factory.register_simple_action("CloseGripper", Arc::new(move || gripper2.close()))?;

	// Trees are created at run-time, but only once at the beginning).
	// The currently supported format is XML.
	// IMPORTANT: When the object "tree" goes out of scope, all the tree components are destroyed
	let mut tree = factory.create_from_text(XML)?;

	// To "execute" a Tree you need to "tick" it.
	// The tick is propagated to the children based on the logic of the tree.
	// In this case, the entire sequence is executed, because all the children
	// of the Sequence return SUCCESS.
	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
async fn build_your_first_tree_with_plugin() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	// Load a plugin and register the Behaviors it contains.
	// This automates the registering step.
	// std::println!("before registration");
	// factory.list_behaviors();
	factory.register_from_plugin("libtest_behaviors")?;
	// std::println!("after registration");
	// factory.list_behaviors();

	let mut tree = factory.create_from_text(XML)?;

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}
