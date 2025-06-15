// Copyright © 2025 Stephan Kunz

//! This test implements the third tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_03_generic_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t03_generic_ports.cpp)
//!

use serial_test::serial;
use test_behaviors::test_nodes::{CalculateGoal, PrintTarget};

use dimas_behavior::{behavior::BehaviorState, factory::BehaviorTreeFactory, register_node};

#[doc(hidden)]
extern crate alloc;

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root">
            <CalculateGoal   goal="{GoalPosition}" />
            <PrintTarget     target="{GoalPosition}" />
            <Script          code="OtherGoal:='-1;3'" />
            <PrintTarget     target="{OtherGoal}" />
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
async fn generic_ports() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_node!(factory, CalculateGoal, "CalculateGoal")?;
	register_node!(factory, PrintTarget, "PrintTarget")?;

	let mut tree = factory.create_from_text(XML)?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}

#[tokio::test]
#[serial]
async fn generic_ports_with_plugin() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("test_behaviors")?;

	let mut tree = factory.create_from_text(XML)?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}
