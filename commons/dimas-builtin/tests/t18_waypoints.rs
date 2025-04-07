// Copyright © 2025 Stephan Kunz

//! This test implements the eighteenth example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t18_waypoints.cpp)
//!

use dimas_behavior::behavior::BehaviorStatus;

#[tokio::test]
#[ignore = "not yet implemented"]
async fn waypoints() -> anyhow::Result<()> {
	let result = BehaviorStatus::Failure;
	println!("not yet implemented");
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
