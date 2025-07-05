// Copyright © 2025 Stephan Kunz

//! This test implements the seventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_07_multiple_xml)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t07_load_multiple_xml.cpp)
//!

#[path = "../common/mod.rs"]
mod common;

use common::test_data::SaySomething;
use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory, register_behavior};

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

// @TODO: implment path problem solution
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// set the necessary directory, currently not working
	let mut dir = std::env::current_dir()?;
	dir.push("examples");
	std::env::set_current_dir(dir)?;

	let mut factory = BehaviorTreeFactory::with_groot2_behaviors()?;

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
