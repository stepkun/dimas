// Copyright © 2024 Stephan Kunz

//! This test implements the twelvth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_12_default_ports)
//!
//! Differences to BehaviorTree.CPP
//! - It is not possible to add an action node directly below the root node
//! - 1 of the 6 ways in BehaviorTree.CPP is currently not working
//!

#[doc(hidden)]
extern crate alloc;

use alloc::{
	fmt::{Display, Formatter},
	str::FromStr,
};
use core::num::ParseIntError;
use dimas_builtin::factory::BTFactory;

use dimas_behavior::{
	behavior::{BehaviorResult, BehaviorStatus, error::BehaviorError},
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <NodeWithDefaultPoints input="-1,-2"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Action ID="NodeWithDefaultPoints"
                editable="true">
            <input_port name="input"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point2D {
	x: i32,
	y: i32,
}

impl Display for Point2D {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

impl FromStr for Point2D {
	type Err = ParseIntError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		// remove redundant ' and &apos; from string
		let s = value
			.replace('\'', "")
			.trim()
			.replace("&apos;", "")
			.trim()
			.to_string();
		let v: Vec<&str> = s.split(',').collect();
		let x = i32::from_str(v[0])?;
		let y = i32::from_str(v[1])?;
		Ok(Self { x, y })
	}
}

/// SyncAction "CalculateGoal"
#[behavior(SyncAction)]
struct NodeWithDefaultPoints {}

#[behavior(SyncAction)]
impl NodeWithDefaultPoints {
	fn ports() -> PortList {
		define_ports!(
			input_port!("input"), // no default value, input is [-1,-2]
			input_port!("pointA", Point2D { x: 1, y: 2 }), // default value is [1,2]
			input_port!("pointB", "{point}"), // default value inside blackboard {pointB}
			input_port!("pointC", "5,6"), // default value is [5,6],
			input_port!("pointD", "{=}"), // default value inside blackboard {pointD}
			input_port!("pointE", r#"(json:{"x':9,"y":10})"#)  // default value is [9,10]
		)
	}

	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config_mut().get_input("input")?;
		let point = Point2D::from_str(&msg)
			.map_err(|_| BehaviorError::ParsePortValue("input".into(), msg))?;
		println!("input:  [{},{}]", point.x, point.y);

		let point: Point2D = bhvr_.config_mut().get_input("pointA")?;
		println!("pointA:  [{},{}]", point.x, point.y);

		let point: Point2D = bhvr_.config_mut().get_input("pointB")?;
		println!("pointB:  [{},{}]", point.x, point.y);

		let msg: String = bhvr_.config_mut().get_input("pointC")?;
		let point = Point2D::from_str(&msg)
			.map_err(|_| BehaviorError::ParsePortValue("pointC".into(), msg))?;
		println!("pointC:  [{},{}]", point.x, point.y);

		let point: Point2D = bhvr_.config_mut().get_input("pointD")?;
		println!("pointD:  [{},{}]", point.x, point.y);

		// let msg: String = bhvr_.config.get_input("pointE")?;
		// dbg!(&msg);
		// let point: Point2D = bhvr_.config.get_input("pointE")?;
		// println!("pointE:  [{},{}]", point.x, point.y);

		Ok(BehaviorStatus::Success)
	}
}

#[tokio::test]
async fn default_ports() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "NodeWithDefaultPoints", NodeWithDefaultPoints);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// initialize blackboard values
	tree.root_blackboard()
		.set("point", Point2D { x: 3, y: 4 });
	tree.root_blackboard()
		.set("pointD", Point2D { x: 7, y: 8 });

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
