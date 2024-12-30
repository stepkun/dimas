// Copyright © 2024 Stephan Kunz

//! This test implements the fifth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_05_subtrees)
//!
//! It is enriched with random behavior of nodes
//! - `IsDoorClosed` in main.rs
//! - `OpenDoor` in subtree.rs
//! - `PickLock` in subtree.rs
//!

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::{behavior, register_action, register_condition};
use rand::Rng;

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
        main_tree_to_execute="MainTree">
    <BehaviorTree ID="DoorClosed">
        <Fallback>
            <OpenDoor/>
            <Retry num_attempts="5">
                <PickLock/>
            </Retry>
            <SmashDoor/>
        </Fallback>
    </BehaviorTree>

    <BehaviorTree ID="MainTree">
        <Sequence>
            <Fallback>
            <Inverter>
                <IsDoorClosed/>
            </Inverter>
            <SubTree ID="DoorClosed"
                _autoremap="false"/>
          </Fallback>
            <PassThroughDoor/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Condition ID="IsDoorClosed"
            editable="true"/>
        <Action ID="OpenDoor"
            editable="true"/>
        <Action ID="PassThroughDoor"
            editable="true"/>
        <Action ID="PickLock"
            editable="true"/>
        <Action ID="SmashDoor"
            editable="true"/>
    </TreeNodesModel>
</root>
"#;

/// Condition "IsDoorClosed"
#[behavior(SyncCondition)]
struct IsDoorClosed {}

#[behavior(SyncCondition)]
impl IsDoorClosed {
	async fn tick(&mut self) -> BehaviorResult {
		let mut rng = rand::thread_rng();
		let state = rng.gen::<bool>();
		if state {
			println!("door is closed");

			Ok(BehaviorStatus::Success)
		} else {
			println!("door is open");

			Ok(BehaviorStatus::Failure)
		}
	}
}

/// SyncAction "PassThroughDoor"
#[behavior(SyncAction)]
struct PassThroughDoor {}

#[behavior(SyncAction)]
impl PassThroughDoor {
	async fn tick(&mut self) -> BehaviorResult {
		println!("door passed");

		Ok(BehaviorStatus::Success)
	}
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register main tree nodes
	register_condition!(factory, "IsDoorClosed", IsDoorClosed);
	register_action!(factory, "PassThroughDoor", PassThroughDoor);
	// register subtrees nodes
	subtree::register_nodes(&mut factory);

	// create the BT
	let mut tree = factory.create_tree(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	println!("tree result is {result}");

	Ok(())
}

/// Implementation of the subtree
mod subtree {
	use super::*;

	/// SyncAction "OpenDoor"
	#[behavior(SyncAction)]
	struct OpenDoor {}

	#[behavior(SyncAction)]
	impl OpenDoor {
		async fn tick(&mut self) -> BehaviorResult {
			let mut rng = rand::thread_rng();
			let state = rng.gen::<bool>();
			if state {
				println!("opened door");

				Ok(BehaviorStatus::Success)
			} else {
				println!("could not open door");

				Ok(BehaviorStatus::Failure)
			}
		}
	}

	/// SyncAction "PickLock"
	#[behavior(SyncAction)]
	struct PickLock {}

	#[behavior(SyncAction)]
	impl PickLock {
		async fn tick(&mut self) -> BehaviorResult {
			let mut rng = rand::thread_rng();
			let state = rng.gen::<i32>();
			if state % 5 == 0 {
				println!("picked lock");

				Ok(BehaviorStatus::Success)
			} else {
				println!("could not pick lock");

				Ok(BehaviorStatus::Failure)
			}
		}
	}

	/// SyncAction "SmashDoor"
	#[behavior(SyncAction)]
	struct SmashDoor {}

	#[behavior(SyncAction)]
	impl SmashDoor {
		async fn tick(&mut self) -> BehaviorResult {
			println!("smashed door");

			Ok(BehaviorStatus::Success)
		}
	}

	pub fn register_nodes(factory: &mut BTFactory) {
		register_action!(factory, "OpenDoor", OpenDoor);
		register_action!(factory, "PickLock", PickLock);
		register_action!(factory, "SmashDoor", SmashDoor);
	}
}
