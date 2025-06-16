// Copyright © 2025 Stephan Kunz

//! This test implements the seventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_07_multiple_xml)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t07_load_multiple_xml.cpp)
//!

use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory, register_behavior};
use test_behaviors::test_nodes::SaySomething;

const XML_MAIN: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <SaySomething message="starting MainTree" />
            <SubTree ID="SubA"/>
            <SubTree ID="SubB"/>
        </Sequence>
    </BehaviorTree>
</root>
"#;

const XML_SUB_A: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="SubA">
        <SaySomething message="Executing SubA" />
    </BehaviorTree>
</root>
"#;

const XML_SUB_B: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="SubB">
        <SaySomething message="Executing SubB" />
    </BehaviorTree>
</root>
"#;

#[tokio::test]
async fn load_multiple_xml() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, SaySomething, "SaySomething")?;

	// Register the behavior tree definitions, but do not instantiate them yet.
	// Order is not important.
	factory.register_behavior_tree_from_text(XML_SUB_A)?;
	factory.register_behavior_tree_from_text(XML_SUB_B)?;
	factory.register_behavior_tree_from_text(XML_MAIN)?;

	//Check that the BTs have been registered correctly
	println!("Registered BehaviorTrees:");
	for bt_name in factory.registered_behavior_trees() {
		println!(" - {bt_name}");
	}

	// You can create the MainTree and the subtrees will be added automatically.
	let mut tree = factory.create_tree("MainTree")?;
	// ... and/or you can create only one of the subtrees
	let mut sub_tree_a = factory.create_tree("SubA")?;
	drop(factory);

	println!("----- MainTree tick ----");
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	println!("----- SubA tick ----");
	sub_tree_a.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}

const XML_WITH_INCLUDE: &str = r#"
<root BTCPP_format="4">
    <include path="./subtree_A.xml" />
    <include path="./subtree_B.xml" />
    <BehaviorTree ID="MainTree">
        <Sequence>
            <SaySomething message="starting MainTree" />
            <SubTree ID="SubA"/>
            <SubTree ID="SubB"/>
        </Sequence>
    </BehaviorTree>
</root>
"#;

#[tokio::test]
async fn load_external_xml() -> anyhow::Result<()> {
	// set the tests directory as current dir for testing purpose
	let mut dir = std::env::current_dir()?;
	dir.push("tests");
	std::env::set_current_dir(dir)?;

	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, SaySomething, "SaySomething")?;

	// Register the behavior tree definition, but do not instantiate the tree yet.
	factory.register_behavior_tree_from_text(XML_WITH_INCLUDE)?;

	//Check that the BTs have been registered correctly
	println!("Registered BehaviorTrees:");
	for bt_name in factory.registered_behavior_trees() {
		println!(" - {bt_name}");
	}

	// You can create the MainTree and the subtrees will be added automatically.
	let mut tree = factory.create_tree("MainTree")?;
	// ... and/or you can create only one of the subtrees
	let mut sub_tree_a = factory.create_tree("SubA")?;
	drop(factory);

	println!("----- MainTree tick ----");
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	println!("----- SubA tick ----");
	sub_tree_a.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}
