// Copyright © 2025 Stephan Kunz

//! Tests the [`Sequence`] behavior

extern crate alloc;

use dimas_behavior::{
	behavior::{
		BehaviorState::{self, *},
		BehaviorStatic,
		action::ChangeStateAfter,
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
		<Sequence name="simple_sequence">
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
async fn simple_sequence(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] input3: BehaviorState,
	#[case] expected: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "Behavior1", BehaviorState::Running, input1, 0)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior2", BehaviorState::Running, input2, 0)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior3", BehaviorState::Running, input3, 0)?;
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
async fn simple_sequence_errors(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] input3: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "Behavior1", BehaviorState::Running, input1, 0)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior2", BehaviorState::Running, input2, 0)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior3", BehaviorState::Running, input3, 0)?;
	register_behavior!(factory, Sequence, "Sequence")?;

	let mut tree = factory.create_from_text(TREE_DEFINITION)?;
	drop(factory);

	let result = tree.tick_once().await;
	assert!(result.is_err());
	Ok(())
}

#[tokio::test]
#[rstest]
#[case(Success, Failure, Running, Running, Running, Success)]
#[case(Failure, Success, Failure, Failure, Failure, Failure)]
async fn simple_sequence_reactiveness1(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] expected1: BehaviorState,
	#[case] expected2: BehaviorState,
	#[case] expected3: BehaviorState,
	#[case] expected4: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "Behavior1", input1, input2, 1)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior2", input1, input2, 2)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior3", input1, input2, 3)?;
	register_behavior!(factory, Sequence, "Sequence")?;

	let mut tree = factory.create_from_text(TREE_DEFINITION)?;
	drop(factory);

	let mut result = tree.tick_once().await?;
	assert_eq!(result, expected1);
	result = tree.tick_once().await?;
	assert_eq!(result, expected2);
	result = tree.tick_once().await?;
	assert_eq!(result, expected3);
	result = tree.tick_once().await?;
	assert_eq!(result, expected4);

	tree.reset().await?;

	result = tree.tick_once().await?;
	assert_eq!(result, expected1);
	result = tree.tick_once().await?;
	assert_eq!(result, expected2);
	result = tree.tick_once().await?;
	assert_eq!(result, expected3);
	result = tree.tick_once().await?;
	assert_eq!(result, expected4);

	Ok(())
}

#[tokio::test]
#[rstest]
#[case(Success, Failure, Running, Running, Running, Success)]
#[case(Failure, Success, Running, Running, Failure, Running)]
async fn simple_sequence_reactiveness2(
	#[case] input1: BehaviorState,
	#[case] input2: BehaviorState,
	#[case] expected1: BehaviorState,
	#[case] expected2: BehaviorState,
	#[case] expected3: BehaviorState,
	#[case] expected4: BehaviorState,
) -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "Behavior1", input1, input2, 3)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior2", input1, input2, 2)?;
	register_behavior!(factory, ChangeStateAfter, "Behavior3", input1, input2, 1)?;
	register_behavior!(factory, Sequence, "Sequence")?;

	let mut tree = factory.create_from_text(TREE_DEFINITION)?;
	drop(factory);

	let mut result = tree.tick_once().await?;
	assert_eq!(result, expected1);
	result = tree.tick_once().await?;
	assert_eq!(result, expected2);
	result = tree.tick_once().await?;
	assert_eq!(result, expected3);
	result = tree.tick_once().await?;
	assert_eq!(result, expected4);

	tree.reset().await?;

	result = tree.tick_once().await?;
	assert_eq!(result, expected1);
	result = tree.tick_once().await?;
	assert_eq!(result, expected2);
	result = tree.tick_once().await?;
	assert_eq!(result, expected3);
	result = tree.tick_once().await?;
	assert_eq!(result, expected4);

	Ok(())
}
