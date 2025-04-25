// Copyright © 2025 Stephan Kunz

//! This test implements the seventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_07_multiple_xml)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t07_load_multiple_xml.cpp)
//!

use dimas_behavior::{behavior::BehaviorStatus, factory::BehaviorTreeFactory};
use serial_test::serial;
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
#[serial]
#[ignore]
async fn load_multiple_xml() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<SaySomething>("SaySomething")?;

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
	println!("----- MainTree tick ----");
	let mut tree = factory.create_tree("MainTree")?;
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	// ... or you can create only one of the subtrees
	println!("----- SubA tick ----");
	let mut sub_tree_a = factory.create_tree("SubA")?;
	sub_tree_a.tick_while_running().await?;

	Ok(())
}
