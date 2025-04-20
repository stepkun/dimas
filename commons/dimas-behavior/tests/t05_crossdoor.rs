// Copyright © 2025 Stephan Kunz

//! This test implements the first tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_01_first_tree)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t01_build_your_first_tree.cpp)
//!

use cross_door::cross_door::CrossDoor;
use dimas_behavior::{factory::NewBehaviorTreeFactory, new_behavior::NewBehaviorStatus};
use serial_test::serial;

const XML: &str = r#"
<root BTCPP_format="4">
	<BehaviorTree ID="MainTree">
        <Sequence>
            <Fallback>
                <Inverter>
                    <IsDoorClosed/>
                </Inverter>
                <SubTree ID="DoorClosed"/>
            </Fallback>
            <PassThroughDoor/>
        </Sequence>
	</BehaviorTree>

    <BehaviorTree ID="DoorClosed">
        <Fallback>
            <OpenDoor/>
            <RetryUntilSuccessful num_attempts="5">
                <PickLock/>
            </RetryUntilSuccessful>
            <SmashDoor/>
        </Fallback>
    </BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
#[ignore]
async fn crossdoor() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	let cross_door = CrossDoor::default();
	cross_door.register_nodes(&mut factory)?;

	// In this example a single XML contains multiple <BehaviorTree>
	// To determine which one is the "main one", we should first register
	// the XML and then allocate a specific tree, using its ID

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree()?;

	// helper function to print the tree
	NewBehaviorTreeFactory::print_tree_recursively(tree.root_node());

	// Tick multiple times, until either FAILURE of SUCCESS is returned
	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
#[serial]
#[ignore]
async fn crossdoor_with_plugin() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("cross_door")?;

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree()?;

	NewBehaviorTreeFactory::print_tree_recursively(tree.root_node());

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}
