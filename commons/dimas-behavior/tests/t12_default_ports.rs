// Copyright © 2025 Stephan Kunz

//! This test implements the twelvth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_12_default_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t12_default_ports.cpp)
//!

extern crate alloc;

use std::{
	fmt::{Display, Formatter},
	num::ParseIntError,
	str::FromStr,
};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{
		BehaviorData, BehaviorError, BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
	},
	blackboard::{BlackboardInterface, SharedBlackboard},
	factory::BehaviorTreeFactory,
	input_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};

const XML: &str = r#"
<root BTCPP_format="4">
  	<BehaviorTree ID="MainTree">
        <Sequence>
            <NodeWithDefaultPoints input="-1,-2"/>
        </Sequence>
  	</BehaviorTree>
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

#[derive(Behavior, Debug, Default)]
struct BehaviorWithDefaultPoints {}

#[async_trait::async_trait]
impl BehaviorInstance for BehaviorWithDefaultPoints {
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let msg: String = blackboard.get("input".into())?;
		let point = Point2D::from_str(&msg).map_err(|_| BehaviorError::ParsePortValue("input".into(), msg.into()))?;
		assert_eq!(point, Point2D { x: -1, y: -2 });
		println!("input:  [{},{}]", point.x, point.y);

		let point: Point2D = blackboard.get("pointA".into())?;
		assert_eq!(point, Point2D { x: 1, y: 2 });
		println!("pointA:  [{},{}]", point.x, point.y);

		let point: Point2D = blackboard.get("pointB".into())?;
		assert_eq!(point, Point2D { x: 3, y: 4 });
		println!("pointB:  [{},{}]", point.x, point.y);

		let msg: String = blackboard.get("pointC".into())?;
		let point = Point2D::from_str(&msg).map_err(|_| BehaviorError::ParsePortValue("pointC".into(), msg.into()))?;
		assert_eq!(point, Point2D { x: 5, y: 6 });
		println!("pointC:  [{},{}]", point.x, point.y);

		let point: Point2D = blackboard.get("pointD".into())?;
		assert_eq!(point, Point2D { x: 7, y: 8 });
		println!("pointD:  [{},{}]", point.x, point.y);

		// @TODO: parsing json
		// let msg: String = blackboard.get("pointE".into())?;
		// dbg!(&msg);
		// let point = Point2D::from_str(&msg).map_err(|_| BehaviorError::ParsePortValue("pointE".into(), msg.into()))?;
		// // let point: Point2D = blackboard.get("pointE".into())?;
		// assert_eq!(point, Point2D{x:9, y:10});
		// println!("pointE:  [{},{}]", point.x, point.y);

		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for BehaviorWithDefaultPoints {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list!(
			input_port!(String, "input"),                           // default value from XML is [-1,-2]
			input_port!(Point2D, "pointA", Point2D { x: 1, y: 2 }), // default value is [1,2]
			input_port!(Point2D, "pointB", "{point}"),              // default value inside blackboard {pointB}
			input_port!(Point2D, "pointC", "5,6"),                  // default value is [5,6],
			input_port!(Point2D, "pointD", "{=}"),                  // default value inside blackboard {pointD}
			input_port!(Point2D, "pointE", r#"(json:{"x':9,"y":10})"#)  // default value is [9,10]
		)
	}
}

#[tokio::test]
async fn default_ports() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<BehaviorWithDefaultPoints>("NodeWithDefaultPoints")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	// initialize blackboard values
	tree.blackboard()
		.set("point".into(), Point2D { x: 3, y: 4 })?;
	tree.blackboard()
		.set("pointD".into(), Point2D { x: 7, y: 8 })?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}
