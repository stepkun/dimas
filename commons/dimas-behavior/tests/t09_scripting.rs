// Copyright © 2025 Stephan Kunz

//! This test implements the nineth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_09_scripting)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t09_scripting.cpp)
//!

use dimas_behavior::{
	behavior::BehaviorStatus, factory::BehaviorTreeFactory
};
use serial_test::serial;
use test_behaviors::test_nodes::SaySomething;

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <Script code=" msg:='hello world' " />
            <Script code=" A:=THE_ANSWER; B:=3.14; color:=RED " />
            <Precondition if="A>-B && color != BLUE" else="FAILURE">
                <Sequence>
                  <SaySomething message="{A}"/>
                  <SaySomething message="{B}"/>
                  <SaySomething message="{msg}"/>
                  <SaySomething message="{color}"/>
                </Sequence>
            </Precondition>
        </Sequence>
    </BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
#[ignore]
async fn scripting() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<SaySomething>("SaySomething")?;

	let mut tree = factory.create_from_text(XML)?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}

#[tokio::test]
#[serial]
#[ignore]
async fn scripting_with_plugin() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("test_behaviors")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
