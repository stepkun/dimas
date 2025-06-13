// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! This test implements the eighteenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t18_waypoints.cpp)
//!

// //! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_11_groot2)

extern crate alloc;

use std::{
	fmt::{Display, Formatter},
	num::ParseIntError,
	str::FromStr,
};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorError, BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::{BlackboardInterface, SharedBlackboard},
	factory::BehaviorTreeFactory,
	input_port,
	output_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};

const XML: &str = r#"
<root BTCPP_format="4">
  	<BehaviorTree ID="MainTree">
  	</BehaviorTree>
</root>
"#;

#[tokio::test]
#[ignore = "not yet available"]
async fn waypoints() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	// factory.register_node_type::<UpdatePosition>("UpdatePosition")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}
