// Copyright © 2025 Stephan Kunz

//! Tests the [`Sequence`] behavior

extern crate alloc;

use dimas_behavior::{
	behavior::{
		BehaviorState::{self, *},
		BehaviorStatic,
		action::StateAfter,
		control::Sequence,
	},
	factory::BehaviorTreeFactory,
	register_behavior,
};

use rstest::rstest;

const TREE_DEFINITION: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="sequence">
			<Behavior1	name="step1"/>
			<Behavior2	name="step2"/>
			<Behavior3	name="step3"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
#[rstest]
#[case(Running, Idle, Idle, Running)]
#[case(Failure, Running, Idle, Failure)]
#[case(Failure, Failure, Running, Failure)]
#[case(Failure, Failure, Failure, Failure)]
#[case(Success, Running, Idle, Running)]
#[case(Success, Success, Running, Running)]
#[case(Success, Failure, Success, Failure)]
#[case(Success, Success, Failure, Failure)]
#[case(Failure, Success, Idle, Failure)]
#[case(Success, Running, Failure, Running)]
#[case(Skipped, Skipped, Success, Success)]
#[case(Skipped, Skipped, Skipped, Skipped)]
#[case(Skipped, Skipped, Running, Running)]
#[case(Skipped, Skipped, Failure, Failure)]
#[case(Success, Skipped, Success, Success)]
#[case(Success, Success, Success, Success)]
async fn sequence(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] input3: BehaviorState,
	#[case] expected: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "Behavior1", input1, 0)?;
	register_behavior!(factory, StateAfter, "Behavior2", input2, 0)?;
	register_behavior!(factory, StateAfter, "Behavior3", input3, 0)?;
	register_behavior!(factory, Sequence, "Sequence")?;

	let mut tree = factory.create_from_text(TREE_DEFINITION)?;
	drop(factory);

	let mut result = tree.tick_once().await?;
	assert_eq!(result, expected);
	result = tree.tick_once().await?;
	assert_eq!(result, expected);
	tree.reset().await?;
	result = tree.tick_once().await?;
	assert_eq!(result, expected);
	Ok(())
}

#[tokio::test]
#[rstest]
#[case(Idle, Idle, Idle)]
#[case(Idle, Success, Idle)]
#[case(Idle, Failure, Idle)]
#[case(Idle, Running, Idle)]
#[case(Idle, Skipped, Idle)]
#[case(Skipped, Skipped, Idle)]
#[case(Success, Idle, Idle)]
async fn sequence_errors(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] input3: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "Behavior1", input1, 0)?;
	register_behavior!(factory, StateAfter, "Behavior2", input2, 0)?;
	register_behavior!(factory, StateAfter, "Behavior3", input3, 0)?;
	register_behavior!(factory, Sequence, "Sequence")?;

	let mut tree = factory.create_from_text(TREE_DEFINITION)?;
	drop(factory);

	let result = tree.tick_once().await;
	assert!(result.is_err());
	Ok(())
}
