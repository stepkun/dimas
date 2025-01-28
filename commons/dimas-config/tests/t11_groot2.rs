// Copyright © 2024 Stephan Kunz

//! This test implements the eleventh tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_11_groot2)
//!

use dimas_core::behavior::BehaviorStatus;

#[tokio::test]
#[ignore]
async fn groot2() -> anyhow::Result<()> {
	let result = BehaviorStatus::Failure;
	println!("not yet implemented");
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
