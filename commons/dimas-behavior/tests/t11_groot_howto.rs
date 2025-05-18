// Copyright © 2025 Stephan Kunz

//! This test implements the eleventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_11_groot2)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t11_groot_howto.cpp)
//!

use dimas_behavior::{behavior::BehaviorStatus, factory::BehaviorTreeFactory};

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="MainTree">
    </BehaviorTree>
</root>
"#;

#[tokio::test]
#[ignore]
async fn groot_howto() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}
