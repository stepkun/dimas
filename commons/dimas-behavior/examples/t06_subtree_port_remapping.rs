// Copyright © 2025 Stephan Kunz

//! This test implements the sixth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://https://www.behaviortree.dev/docs/tutorial-basics/tutorial_06_subtree_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t06_subtree_port_remapping.cpp)
//!

mod common;

use common::test_data::{MoveBaseAction, SaySomething};
use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory, register_behavior};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_groot2_behaviors()?;

	register_behavior!(factory, SaySomething, "SaySomething")?;
	register_behavior!(factory, MoveBaseAction, "MoveBase")?;

	factory.register_behavior_tree_from_text(XML)?;
	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	println!("------ Root BB ------");
	// @TODO: tree.subtree(0)?.blackboard().debug_message();
	println!("----- Second BB -----");
	// @TODO: tree.subtree(1)?.blackboard().debug_message();
	Ok(())
}
