// Copyright © 2024 Stephan Kunz

//! This test implements the third tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_03_generic_ports)
//!
//! Differences to BehaviorTree.CPP
//! - there is no Script node available, that has to be implemented by user
//!

#[doc(hidden)]
extern crate alloc;

use core::{num::ParseFloatError, str::FromStr};

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus, error::BehaviorError},
	blackboard::FromString,
	define_ports, input_port, output_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"
<root BTCPP_format="4"
        main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
       <Sequence  name="root">
           <CalculateGoal goal="{GoalPosition}" />
           <PrintTarget   target="{GoalPosition}" />
           <Script        code=" OtherGoal:=&apos;-1;3&apos; " />
           <PrintTarget   target="{OtherGoal}" />
       </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Action ID="CalculateGoal"
                editable="true">
            <output_port name="goal"/>
        </Action>
        <Action ID="PrintTarget"
                editable="true">
            <input_port name="target"/>
        </Action>
        <Action ID="Script"
                editable="true">
            <input_port name="code"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position2D {
	x: f64,
	y: f64,
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
		let x = f64::from_str(v[0])?;
		let y = f64::from_str(v[1])?;
		Ok(Self { x, y })
	}
}

/// SyncAction "CalculateGoal"
#[behavior(SyncAction)]
struct CalculateGoal {}

#[behavior(SyncAction)]
impl CalculateGoal {
	fn ports() -> PortList {
		define_ports!(output_port!("goal"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		// initialize GoalPosition
		let pos = Position2D { x: 1.1, y: 2.3 };
		bhvr_.config.set_output("goal", pos)?;

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "PrintTarget"
#[behavior(SyncAction)]
struct PrintTarget {}

#[behavior(SyncAction)]
impl PrintTarget {
	fn ports() -> PortList {
		define_ports!(input_port!("target"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let pos: Position2D = bhvr_.config.get_input("target")?;

		println!("Target positions: [ {}, {} ]", pos.x, pos.y);

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "Script"
#[behavior(SyncAction)]
struct Script {}

#[behavior(SyncAction)]
impl Script {
	fn ports() -> PortList {
		define_ports!(input_port!("code"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let script: String = bhvr_.config.get_input("code")?;
		let elements: Vec<&str> = script.split(":=").collect();
		let pos = Position2D::from_string(elements[1].trim()).map_err(|_| {
			BehaviorError::ParsePortValue("code".to_string(), "Position2D".to_string())
		})?;
		bhvr_
			.config
			.blackboard()
			.to_owned()
			.set(elements[0].trim(), pos);

		Ok(BehaviorStatus::Success)
	}
}

#[tokio::test]
async fn generic_ports() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "CalculateGoal", CalculateGoal);
	register_action!(factory, "PrintTarget", PrintTarget);
	register_action!(factory, "Script", Script);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);
	let pos: Position2D = factory
		.blackboard()
		.get("GoalPosition")
		.expect("GoalPosition not found");
	assert_eq!(pos, Position2D { x: 1.1, y: 2.3 });
	let pos: Position2D = factory
		.blackboard()
		.get("OtherGoal")
		.expect("OtherGoal not found");
	assert_eq!(pos, Position2D { x: -1.0, y: 3.0 });

	Ok(())
}
