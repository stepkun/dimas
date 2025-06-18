// Copyright © 2025 Stephan Kunz

//! This test implements the first tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_01_first_tree)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t01_build_your_first_tree.cpp)
//!

extern crate alloc;
mod test_data;

use dimas_behavior::{
	behavior::{BehaviorState, BehaviorType},
	factory::BehaviorTreeFactory,
	register_behavior,
};
use test_data::{ApproachObject, GripperInterface, check_battery};

/// This definition uses implicit node ID's
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
async fn build_your_first_tree_implicit() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	// The recommended way to create a Behavior is through inheritance/composition.
	// Even if it requires more boilerplate, it allows you to use more functionalities
	// like ports (we will discuss this in future tutorials).
	register_behavior!(factory, ApproachObject, "ApproachObject")?;

	// Registering a SimpleAction/SimpleCondition using a function pointer.
	register_behavior!(factory, check_battery, "CheckBattery", BehaviorType::Condition)?;

	// You can also create SimpleAction/SimpleCondition using methods of a struct.
	register_behavior!(
		factory,
		GripperInterface::default(),
		open,
		"OpenGripper",
		BehaviorType::Action,
		close,
		"CloseGripper",
		BehaviorType::Action,
	)?;

	// Trees are created at run-time, but only once at the beginning.
	// The currently supported format is XML.
	// IMPORTANT: When the object "tree" goes out of scope, all the tree components are destroyed
	let mut tree = factory.create_from_text(XML)?;
	// dropping the factory to free memory
	drop(factory);

	// To "execute" a Tree you need to "tick" it.
	// The tick is propagated to the children based on the logic of the tree.
	// In this case, the entire sequence is executed, because all the children
	// of the Sequence return SUCCESS.
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}

/// This definition uses explicit node ID's
const XML_EXPLICIT: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Control ID="Sequence" name="root_sequence">
			<Condition ID="CheckBattery"	name="battery_ok"/>
			<Action ID="OpenGripper"		name="open_gripper"/>
			<Action ID="ApproachObject"		name="approach_object"/>
			<Action ID="CloseGripper"		name="close_gripper"/>
		</Control>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn build_your_first_tree_explicit() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	// The recommended way to create a Behavior is through inheritance/composition.
	// Even if it requires more boilerplate, it allows you to use more functionalities
	// like ports (we will discuss this in future tutorials).
	register_behavior!(factory, ApproachObject, "ApproachObject")?;

	// Registering a SimpleAction/SimpleCondition using a function pointer.
	register_behavior!(factory, check_battery, "CheckBattery", BehaviorType::Condition)?;

	// You can also create SimpleAction/SimpleCondition using methods of a struct.
	register_behavior!(
		factory,
		GripperInterface::default(),
		open,
		"OpenGripper",
		BehaviorType::Action,
		close,
		"CloseGripper",
		BehaviorType::Action,
	)?;

	let mut tree = factory.create_from_text(XML_EXPLICIT)?;
	// dropping the factory to free memory
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}
