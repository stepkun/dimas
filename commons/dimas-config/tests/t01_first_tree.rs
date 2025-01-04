// Copyright © 2024 Stephan Kunz

//! This test implements the first tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_01_first_tree)
//!
//! Differences to BehaviorTree.CPP:
//! - we cannot register functions/methods of a struct/class
//! - port `name` is not available by default
//!

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::{behavior, register_action, register_condition};

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence>
			<CheckBattery/>
			<OpenGripper/>
			<ApproachObject/>
			<CloseGripper/>
		</Sequence>
	</BehaviorTree>

	<!-- Description of Node Models (used by Groot) -->
	<TreeNodesModel>
		<Condition ID="CheckBattery"
				editable="true"/>
		<Action ID="ApproachObject"
				editable="true"/>
		<Action ID="CloseGripper"
				editable="true"/>
		<Action ID="OpenGripper"
				editable="true"/>
	</TreeNodesModel>
</root>
"#;

/// Condition "CheckBattery"
#[behavior(SyncCondition)]
struct CheckBattery {}

#[behavior(SyncCondition)]
impl CheckBattery {
	async fn tick(&mut self) -> BehaviorResult {
		println!("battery state is ok");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "OpenGripper"
#[behavior(SyncAction)]
struct OpenGripper {}

#[behavior(SyncAction)]
impl OpenGripper {
	async fn tick(&mut self) -> BehaviorResult {
		println!("opened gripper");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "ApproachObject"
#[behavior(SyncAction)]
struct ApproachObject {}

#[behavior(SyncAction)]
impl ApproachObject {
	async fn tick(&mut self) -> BehaviorResult {
		println!("approaching object");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "CloseGripper"
#[behavior(SyncAction)]
struct CloseGripper {}

#[behavior(SyncAction)]
impl CloseGripper {
	async fn tick(&mut self) -> BehaviorResult {
		println!("closed gripper");

		Ok(BehaviorStatus::Success)
	}
}

#[tokio::test]
async fn first_tree() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_condition!(factory, "CheckBattery", CheckBattery);
	register_action!(factory, "OpenGripper", OpenGripper);
	register_action!(factory, "ApproachObject", ApproachObject);
	register_action!(factory, "CloseGripper", CloseGripper);

	// create the BT
	let mut tree = factory.create_tree(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
