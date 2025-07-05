// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! This test implements the fifteenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_15_replace_rules)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t15_nodes_mocking.cpp)
//!

extern crate alloc;

use std::{
	fmt::{Display, Formatter},
	num::ParseIntError,
	str::FromStr,
};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorError, BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	blackboard::{BlackboardInterface, SharedBlackboard},
	factory::BehaviorTreeFactory,
	input_port, output_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};

const XML: &str = r#"
<root BTCPP_format="4">
  	<BehaviorTree ID="MainTree">
		<AlwaysSuccess/>
  	</BehaviorTree>
</root>
"#;

// @TODO: implement
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_groot2_behaviors()?;

	// register_behavior!(factory, SaySomething, "SaySomething")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}
