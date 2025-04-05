// Copyright © 2024 Stephan Kunz

//! This test implements the fifteenth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_15_replace_rules)
//!

use dimas_behavior::behavior::BehaviorStatus;

#[tokio::test]
#[ignore = "not yet implemented"]
async fn replace_rules() -> anyhow::Result<()> {
	let result = BehaviorStatus::Failure;
	println!("not yet implemented");
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
