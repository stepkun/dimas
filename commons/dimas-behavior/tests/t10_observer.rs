// Copyright © 2025 Stephan Kunz

//! This test implements the tenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_10_observer)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t10_observer.cpp)
//!

extern crate alloc;

use dimas_behavior::{behavior::BehaviorStatus, factory::BehaviorTreeFactory};
use test_behaviors::test_nodes::{AlwaysFailure, AlwaysSuccess};

const XML: &str = r#"
<root BTCPP_format="4">

    <BehaviorTree ID="MainTree">
        <Sequence>
            <Fallback>
                <AlwaysFailure name="failing_action"/>
                <SubTree ID="SubTreeA" name="mysub"/>
            </Fallback>
            <AlwaysSuccess name="last_action"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="SubTreeA">
        <Sequence>
            <AlwaysSuccess name="action_subA"/>
            <SubTree ID="SubTreeB" name="sub_nested"/>
            <SubTree ID="SubTreeB" />
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="SubTreeB">
        <AlwaysSuccess name="action_subB"/>
    </BehaviorTree>

</root>
"#;

#[tokio::test]
#[ignore]
async fn observer() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<AlwaysFailure>("AlwaysFailure")?;
	factory.register_node_type::<AlwaysSuccess>("AlwaysSuccess")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	// Print the unique ID and the corresponding human readable path
	// Path is also expected to be unique.
	for node in tree.iter() {
		println!("{} <-> {}", node.uid(), node.path());
	}

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn observer_with_plugin() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("test_behaviors")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	// Print the unique ID and the corresponding human readable path
	// Path is also expected to be unique.
	for node in tree.iter() {
		println!("{} <-> {}", node.uid(), node.path());
	}

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
