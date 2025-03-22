// Copyright © 2024 Stephan Kunz

//! This test implements the thirteenth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_13_blackboard_reference)
//!
//! Differences to BehaviorTree.CPP
//! - example at behaviorTree.CPP is inconsistent, does not match code in github repo
//! - could not get the `get_exact::<wanted_type>()` access to work
//!

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus},
	define_ports, input_port, output_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

const XML: &str = r#"
<root BTCPP_format="4"
      main_tree_to_execute="SegmentCup">
    <BehaviorTree ID="SegmentCup">
       <Sequence>
           <AcquirePointCloud  cloud="{pointcloud}"/>
           <SegmentObject  obj_name="cup" cloud="{pointcloud}" obj_pose="{pose}"/>
       </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Action ID="AcquirePointCloud"
                editable="true">
            <output_port name="cloud"/>
        </Action>
        <Action ID="SegmentObject"
                editable="true">
            <input_port name="cloud"/>
            <input_port name="obj_name"/>
            <output_port name="obj_pose"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Point {
	x: i32,
	y: i32,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct PointCloud {
	points: Vec<Point>,
}

/// ActionNode "AcquirePointCloud"
#[behavior(SyncAction)]
struct AcquirePointCloud {}

#[behavior(SyncAction)]
impl AcquirePointCloud {
	fn ports() -> PortList {
		define_ports!(output_port!("cloud"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		println!("setting PointCloud");
		// put a PointCloud into blackboard
		let p_cloud = vec![
			Point { x: 0, y: 0 },
			Point { x: 1, y: 1 },
			Point { x: 2, y: 2 },
			Point { x: 3, y: 3 },
		];
		bhvr_
			.config_mut()
			.blackboard_mut()
			.set("cloud", p_cloud);

		Ok(BehaviorStatus::Success)
	}
}

/// ActionNode "SegmentObject"
#[behavior(SyncAction)]
struct SegmentObject {}

#[behavior(SyncAction)]
impl SegmentObject {
	fn ports() -> PortList {
		define_ports!(
			input_port!("cloud"),
			input_port!("obj_name"),
			output_port!("obj_pose")
		)
	}

	async fn tick(&mut self) -> BehaviorResult {
		println!("accessing PointCloud");
		// let p_cloud = bhvr_
		// 	.config
		// 	.blackboard
		// 	.get_exact::<&PointCloud>("cloud")
		// 	.ok_or_else(|| BehaviorError::FindPort("cloud".into(), "in SegmentObject".into()))?;
		// println!("PointCloud is {p_cloud:#?}");

		// for now it is a failure
		Ok(BehaviorStatus::Failure)
	}
}

#[tokio::test]
#[ignore = "not yet implemented"]
async fn blackboard_reference() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "AcquirePointCloud", AcquirePointCloud);
	register_action!(factory, "SegmentObject", SegmentObject);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
