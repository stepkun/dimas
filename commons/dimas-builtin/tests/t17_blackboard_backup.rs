// Copyright © 2025 Stephan Kunz

//! This test implements the seventeenth example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t17_blackboard_backup.cpp)
//!

use dimas_behavior::behavior::BehaviorStatus;

#[tokio::test]
#[ignore = "not yet implemented"]
async fn backup() -> anyhow::Result<()> {
	let result = BehaviorStatus::Failure;
	println!("not yet implemented");
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
