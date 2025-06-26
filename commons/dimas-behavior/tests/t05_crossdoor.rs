// Copyright © 2025 Stephan Kunz

//! This test implements the fifth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_05_subtrees)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t01_build_your_first_tree.cpp)
//!

mod cross_door;

use cross_door::CrossDoor;
use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory};

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
async fn crossdoor() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_groot2_behaviors()?;

	CrossDoor::register_behaviors(&mut factory)?;

	// In this example a single XML contains multiple <BehaviorTree>
	// To determine which one is the "main one", we should first register
	// the XML and then allocate a specific tree, using its ID
	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_main_tree()?;
	drop(factory);

	// helper function to print the tree
	tree.print()?;

	// Tick multiple times, until either FAILURE of SUCCESS is returned
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}
