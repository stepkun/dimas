// Copyright © 2025 Stephan Kunz

//! Tests the [`ReactiveFallback`]

extern crate alloc;

use dimas_behavior::{
	behavior::{BehaviorState, BehaviorStatic, action::ChangeStateAfter, control::ReactiveFallback},
	factory::BehaviorTreeFactory,
	register_behavior,
};

const SUCCESS: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveFallback name="reactive_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysSuccess	name="step3"/>
		</ReactiveFallback>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn success() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "AlwaysFailure", BehaviorState::Running, BehaviorState::Failure, 3).expect("snh");
	register_behavior!(factory, ChangeStateAfter, "AlwaysSuccess", BehaviorState::Running, BehaviorState::Success, 3).expect("snh");
	register_behavior!(factory, ReactiveFallback, "ReactiveFallback").expect("snh");

	let mut tree = factory.create_from_text(SUCCESS)?;
	drop(factory);

	let mut result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	tree.reset().await.expect("snh");
	result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}

const FAILURE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveFallback name="reactive_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure	name="step3"/>
		</ReactiveFallback>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn failure() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "AlwaysFailure", BehaviorState::Running, BehaviorState::Failure, 3).expect("snh");
	register_behavior!(factory, ReactiveFallback, "ReactiveFallback").expect("snh");

	let mut tree = factory.create_from_text(FAILURE)?;
	drop(factory);

	let mut result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Failure);
	tree.reset().await.expect("snh");
	result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Failure);
	Ok(())
}
