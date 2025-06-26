// Copyright © 2025 Stephan Kunz

//! Tests the [`ReactiveSequence`]

extern crate alloc;

use dimas_behavior::{
	behavior::{BehaviorState, BehaviorStatic, action::StateAfter, control::ReactiveSequence},
	factory::BehaviorTreeFactory,
	register_behavior,
};

const SUCCESS: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveSequence name="reactive_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
		</ReactiveSequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn success() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 3).expect("snh");
	register_behavior!(factory, ReactiveSequence, "ReactiveSequence").expect("snh");

	let mut tree = factory.create_from_text(SUCCESS)?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	Ok(())
}

const FAILURE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveSequence name="reactive_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysFailure	name="step3"/>
		</ReactiveSequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn failure() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 3).expect("snh");
	register_behavior!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 3).expect("snh");
	register_behavior!(factory, ReactiveSequence, "ReactiveSequence").expect("snh");

	let mut tree = factory.create_from_text(FAILURE)?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Failure);
	Ok(())
}
