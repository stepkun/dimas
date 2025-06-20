// Copyright © 2025 Stephan Kunz

//! This test implements the fourth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_04_sequence)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t04_reactive_sequence.cpp)
//!

mod test_data;

use std::time::Duration;

use test_data::{MoveBaseAction, SaySomething, check_battery};

use dimas_behavior::{
	behavior::{BehaviorKind, BehaviorState},
	factory::BehaviorTreeFactory,
	register_behavior,
};

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

#[tokio::test]
async fn std_sequence() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, check_battery, "BatteryOK", BehaviorKind::Condition)?;
	register_behavior!(factory, MoveBaseAction, "MoveBase")?;
	register_behavior!(factory, SaySomething, "SaySomething")?;

	let mut tree = factory.create_from_text(XML)?;
	drop(factory);

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == BehaviorState::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}

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
async fn reactive_sequence() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, check_battery, "BatteryOK", BehaviorKind::Condition)?;
	register_behavior!(factory, MoveBaseAction, "MoveBase")?;
	register_behavior!(factory, SaySomething, "SaySomething")?;

	let mut tree = factory.create_from_text(XML_REACTIVE)?;

	// run the BT using own loop with sleep to avoid busy loop
	let mut result = tree.tick_once().await?;
	while result == BehaviorState::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		result = tree.tick_once().await?;
	}
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}
