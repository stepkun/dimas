// Copyright © 2024 Stephan Kunz

//! This test implements the tenth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_10_observer)
//!
//! Currently not implemented

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::{behavior::{BehaviorResult, BehaviorStatus}, define_ports, input_port, port::PortList};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"
<root BTCPP_format="4"
    main_tree_to_execute="MainTree">
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

/// Action "AlwaysSuccess"
#[behavior(Action)]
struct AlwaysSuccess {}

#[behavior(Action)]
impl AlwaysSuccess {
	fn ports() -> PortList {
		define_ports!(input_port!("name"))
	}

	async fn on_start(&self) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}

	async fn on_running(&self) -> BehaviorResult {
		//println!("ticking AlwaysSuccess");
		Ok(BehaviorStatus::Success)
	}
}


/// Action "AlwaysFailure"
#[behavior(Action)]
struct AlwaysFailure {}

#[behavior(Action)]
impl AlwaysFailure {
	fn ports() -> PortList {
		define_ports!(input_port!("name"))
	}

	async fn on_start(&self) -> BehaviorResult {
		Ok(BehaviorStatus::Failure)
	}

	async fn on_running(&self) -> BehaviorResult {
		Ok(BehaviorStatus::Failure)
	}
}


#[tokio::test]
#[ignore]
async fn observer() -> anyhow::Result<()> {
    // create BT environment
    let mut factory = BTFactory::default();

    // register actons
    register_action!(factory, "AlwaysFailure", AlwaysFailure);
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

    // create the BT
    let mut tree = factory.create_tree(XML)?;

    // run the BT
    let _result = tree.tick_while_running().await?;
    let result = BehaviorStatus::Failure;
    println!("not yet implemented");
	assert_eq!(result, BehaviorStatus::Success);

    Ok(())
}
