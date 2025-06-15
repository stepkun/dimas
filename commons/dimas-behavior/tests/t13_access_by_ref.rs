// Copyright © 2025 Stephan Kunz

//! This test implements the thirteenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_13_blackboard_reference)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t13_access_by_ref.cpp)
//!

extern crate alloc;

use std::{
	fmt::{Display, Formatter},
	num::ParseIntError,
	str::FromStr,
};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorData, BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::{BlackboardInterface, SharedBlackboard},
	factory::BehaviorTreeFactory,
	input_port, output_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};

const XML: &str = r#"
<root BTCPP_format="4">
     <BehaviorTree ID="SegmentCup">
       <Sequence>
           <AcquirePointCloud  cloud="{pointcloud}"/>
           <SegmentObject  obj_name="cup" cloud="{pointcloud}" obj_pose="{pose}"/>
       </Sequence>
    </BehaviorTree>
/root>
"#;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Display for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

impl FromStr for Point {
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct PointCloud {
	points: Vec<Point>,
}

impl Display for PointCloud {
	fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
		todo!()
		// write!(f, "{:?}", self.points)
	}
}

impl FromStr for PointCloud {
	type Err = ParseIntError;

	fn from_str(_value: &str) -> Result<Self, Self::Err> {
		todo!()
	}
}

/// Behavior `AcquirePointCloud`
#[derive(Behavior, Debug, Default)]
struct AcquirePointCloud {}

#[async_trait::async_trait]
impl BehaviorInstance for AcquirePointCloud {
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("setting PointCloud");
		// put a PointCloud into blackboard
		let p_cloud: PointCloud = PointCloud {
			points: vec![
				Point { x: 0, y: 0 },
				Point { x: 1, y: 1 },
				Point { x: 2, y: 2 },
				Point { x: 3, y: 3 },
			],
		};

		blackboard.set("cloud".into(), p_cloud)?;

		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for AcquirePointCloud {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list!(output_port!(PointCloud, "cloud"))
	}
}

/// Behavior `SegmentObject`
#[derive(Behavior, Debug, Default)]
struct SegmentObject();

#[async_trait::async_trait]
impl BehaviorInstance for SegmentObject {
	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("accessing PointCloud");
		// let p_cloud = blackboard
		// 	.get_exact::<&PointCloud>("cloud")
		// 	.ok_or_else(|| BehaviorError::FindPort("cloud".into(), "in SegmentObject".into()))?;
		// println!("PointCloud is {p_cloud:#?}");

		// for now it is a failure
		Ok(BehaviorState::Failure)
	}
}

impl BehaviorStatic for SegmentObject {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list!(
			input_port!(PointCloud, "cloud"),
			input_port!(String, "obj_name"),
			output_port!(String, "obj_pose")
		)
	}
}

#[tokio::test]
#[ignore]
async fn access_by_ref() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<AcquirePointCloud>("AcquirePointCloud")?;
	factory.register_node_type::<SegmentObject>("SegmentObject")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}
