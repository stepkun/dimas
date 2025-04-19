// Copyright © 2025 Stephan Kunz

//! This test implements the fourth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_04_sequence)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t04_reactive_sequence.cpp)
//!

use std::{sync::Arc, time::Duration};

use serial_test::serial;
use test_behaviors::test_nodes::{MoveBaseAction, SaySomething, check_battery};

use dimas_behavior::{factory::NewBehaviorTreeFactory, new_behavior::NewBehaviorStatus};

#[doc(hidden)]
extern crate alloc;

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
        <Sequence name="std root sequence">
            <BatteryOK/>
            <SaySomething   message="mission started..." />
            <MoveBase       goal="1;2;3"/>
            <SaySomething   message="mission completed!" />
        </Sequence>
	</BehaviorTree>
</root>
"#;

const XML_REACTIVE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveSequence name="reactive root sequence">
            <BatteryOK/>
            <Sequence name = "inner std sequence">
                <SaySomething   message="mission started..." />
                <MoveBase       goal="1;2;3"/>
                <SaySomething   message="mission completed!" />
            </Sequence>
		</ReactiveSequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
#[serial]
async fn std_sequence() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_simple_condition("BatteryOK", Arc::new(check_battery))?;
	factory.register_node_type::<MoveBaseAction>("MoveBase")?;
	factory.register_node_type::<SaySomething>("SaySomething")?;

	let mut tree = factory.create_from_text(XML)?;

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == NewBehaviorStatus::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
#[serial]
async fn reactive_sequence() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_simple_condition("BatteryOK", Arc::new(check_battery))?;
	factory.register_node_type::<MoveBaseAction>("MoveBase")?;
	factory.register_node_type::<SaySomething>("SaySomething")?;

	let mut tree = factory.create_from_text(XML_REACTIVE)?;

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == NewBehaviorStatus::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
#[serial]
async fn std_sequence_with_plugin() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("libtest_behaviors")?;

	let mut tree = factory.create_from_text(XML)?;

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == NewBehaviorStatus::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

#[tokio::test]
#[serial]
async fn reactive_sequence_with_plugin() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_from_plugin("libtest_behaviors")?;

	let mut tree = factory.create_from_text(XML_REACTIVE)?;

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == NewBehaviorStatus::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}
