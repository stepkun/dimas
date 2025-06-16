// Copyright © 2025 Stephan Kunz

//! This test implements the sixth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://https://www.behaviortree.dev/docs/tutorial-basics/tutorial_06_subtree_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t06_subtree_port_remapping.cpp)
//!

use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory, register_behavior};
use serial_test::serial;
use test_behaviors::test_nodes::{MoveBaseAction, SaySomething};

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <Script code=" move_goal:='1;2;3' " />
            <SubTree ID="MoveRobot" target="{move_goal}" result="{move_result}" />
            <SaySomething message="{move_result}"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="MoveRobot">
        <Fallback>
            <Sequence>
                <MoveBase  goal="{target}"/>
                <Script code=" result:='goal reached' " />
            </Sequence>
            <ForceFailure>
                <Script code=" result:='error' " />
            </ForceFailure>
        </Fallback>
    </BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
async fn subtree_port_remapping() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, SaySomething, "SaySomething")?;
	register_behavior!(factory, MoveBaseAction, "MoveBase")?;

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	println!("\n------ Root BB ------");
	// @TODO: tree.subtree(0)?.blackboard().debug_message();
	println!("\n----- Second BB -----");
	// @TODO: tree.subtree(1)?.blackboard().debug_message();
	Ok(())
}

#[tokio::test]
#[serial]
async fn subtree_port_remapping_with_plugin() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("test_behaviors")?;

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	println!("\n------ Root BB ------");
	// tree.subtree(0)?.blackboard().debug_message();
	println!("\n----- Second BB -----");
	// tree.subtree(1)?.blackboard().debug_message();
	Ok(())
}

#[tokio::test]
#[ignore]
async fn subtree_blackboard_access_reminder() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("test_behaviors")?;

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	println!("\n------ Root BB ------");
	// tree.subtree(0)?.blackboard().debug_message();
	println!("\n----- Second BB -----");
	// tree.subtree(1)?.blackboard().debug_message();
	Ok(())
}
