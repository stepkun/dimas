// Copyright © 2024 Stephan Kunz

//! This test implements the fourteenth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_14_subtree_model)
//!
//! Differences to BehaviorTree.CPP
//! - there is no Script node available, that has to be implemented by user
//! - example in BehaviorTree.CPP is inconsistent
//! - not sure wether this example really shows how to do it
//!

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus, error::BehaviorError},
	blackboard::FromString,
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <Script code="target:=&apos;1;2;3&apos;"/>
            <SubTree ID="MoveRobot"
                _autoremap="true"
                frame="world"/>
            <SaySomething msg="{result}"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="MoveRobot">
        <Fallback>
            <Sequence>
                <MoveBase goal="{target}"/>
                <Script code="result:=&apos;goal_reached&apos;"/>
            </Sequence>
            <ForceFailure>
                <Script code="result:=&apos;error&apos;"/>
            </ForceFailure>
        </Fallback>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
    <Action ID="MoveBase"
            editable="true">
        <input_port name="goal"/>
    </Action>
    <Action ID="SaySomething"
            editable="true">
        <input_port name="msg"/>
    </Action>
  </TreeNodesModel>

</root>
"#;

/// SyncAction "Script"
#[behavior(SyncAction)]
struct Script {}

#[behavior(SyncAction)]
impl Script {
	async fn tick(&mut self) -> BehaviorResult {
		let script: String = bhvr_.config.get_input("code")?;
		let elements: Vec<&str> = script.split(":=").collect();
		if elements[1].contains('{') {
			let pos = move_robot::Position2D::from_string(elements[1].trim()).map_err(|_| {
				BehaviorError::ParsePortValue("code".to_string(), "Position2D".to_string())
			})?;
			bhvr_
				.config
				.blackboard()
				.to_owned()
				.set(elements[0].trim(), pos);
		} else {
			let mut content = elements[1].to_string();
			// remove redundant ' from string
			content = content.replace('\'', "").trim().to_string();
			// remove redundant &apos; from string
			content = content.replace("&apos;", "").trim().to_string();
			bhvr_
				.config
				.blackboard()
				.to_owned()
				.set(elements[0].trim(), content);
		}

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("code"))
	}
}

/// SyncAction "SaySomething"
#[behavior(SyncAction)]
struct SaySomething {}

#[behavior(SyncAction)]
impl SaySomething {
	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config.get_input("msg")?;

		println!("Robot says: {msg}");

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("msg", "hello"))
	}
}

#[tokio::test]
async fn subtree_model() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::extended();

	// register main tree nodes
	register_action!(factory, "Script", Script);
	register_action!(factory, "SaySomething", SaySomething);
	// register subtrees nodes
	move_robot::register_nodes(&mut factory);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}

/// Implementation of `MoveRobot` tree
mod move_robot {
	use std::{num::ParseFloatError, str::FromStr};

	use dimas_core::blackboard::FromString;

	use super::*;

	#[derive(Clone, Copy, Debug)]
	pub struct Position2D {
		x: f64,
		y: f64,
		theta: f64,
	}

	impl FromString for Position2D {
		type Err = ParseFloatError;

		fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err> {
			value.as_ref().parse()
		}
	}

	impl FromStr for Position2D {
		type Err = ParseFloatError;

		fn from_str(value: &str) -> Result<Self, Self::Err> {
			// remove redundant ' and &apos; from string
			let s = value
				.replace('\'', "")
				.trim()
				.replace("&apos;", "")
				.trim()
				.to_string();
			let v: Vec<&str> = s.split(';').collect();
			let x = f64::from_string(v[0])?;
			let y = f64::from_string(v[1])?;
			let theta = f64::from_string(v[2])?;
			Ok(Self { x, y, theta })
		}
	}

	/// Action "MoveBase"
	#[behavior(Action)]
	struct MoveBase {
		#[bhvr(default)]
		counter: usize,
	}

	#[behavior(Action)]
	impl MoveBase {
		async fn on_start(&mut self) -> BehaviorResult {
			let pos = bhvr_.config.get_input::<Position2D>("goal")?;

			println!(
				"[ MoveBase: SEND REQUEST ]. goal: x={:2.1} y={:2.1} theta={:2.1}",
				pos.x, pos.y, pos.theta
			);

			Ok(BehaviorStatus::Running)
		}

		async fn on_running(&mut self) -> BehaviorResult {
			if self.counter < 5 {
				self.counter += 1;
				println!("--- status: RUNNING");
				Ok(BehaviorStatus::Running)
			} else {
				println!("[ MoveBase: FINISHED ]");
				Ok(BehaviorStatus::Success)
			}
		}

		fn ports() -> PortList {
			define_ports!(input_port!("goal"))
		}
	}

	pub fn register_nodes(factory: &mut BTFactory) {
		register_action!(factory, "MoveBase", MoveBase);
	}
}
