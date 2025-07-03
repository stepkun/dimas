// Copyright © 2025 Stephan Kunz

//! Tests the [`BehaviorTreeObserver`]

extern crate alloc;

use dimas_behavior::{
	behavior::{BehaviorState, BehaviorStatic, action::ChangeStateAfter, control::Fallback},
	factory::BehaviorTreeFactory,
	register_behavior,
	tree::observer::tree_observer::BehaviorTreeObserver,
};

const TREE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Fallback name="observer">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysSuccess	name="step3"/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn tree_observer() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "AlwaysFailure", BehaviorState::Running, BehaviorState::Failure, 3).expect("snh");
	register_behavior!(factory, ChangeStateAfter, "AlwaysSuccess", BehaviorState::Running, BehaviorState::Success, 3).expect("snh");
	register_behavior!(factory, Fallback, "Fallback").expect("snh");

	let mut tree = factory.create_from_text(TREE)?;
	let observer = BehaviorTreeObserver::new(&mut tree);
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);
	// AlwaySuccess should change state 3 times
	assert_eq!(
		observer
			.get_statistics(4)
			.expect("snh")
			.transitions_count,
		3
	);
	// AlwayFailure should change state 3 times
	assert_eq!(
		observer
			.get_statistics(4)
			.expect("snh")
			.transitions_count,
		3
	);
	// The tree should change state 3 times
	assert_eq!(
		observer
			.get_statistics(0)
			.expect("snh")
			.transitions_count,
		3
	);
	Ok(())
}
