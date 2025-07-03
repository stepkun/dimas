// Copyright © 2025 Stephan Kunz

//! Tests a complex [`BehaviorTree`]

extern crate alloc;

use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::ChangeStateAfter,
		control::{
			Fallback, Parallel, ParallelAll, ReactiveFallback, ReactiveSequence, Sequence, SequenceWithMemory,
			WhileDoElse,
		},
	},
	factory::BehaviorTreeFactory,
	register_behavior,
};

const TREE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Fallback name="root_fallback">
			<ParallelAll>
				<Sequence>
					<AlwaysSuccess/>
					<Fallback>
						<AlwaysFailure/>
						<AlwaysFailure/>
						<AlwaysFailure/>
						<AlwaysSuccess/>
					</Fallback>
					<AlwaysSuccess/>
				</Sequence>
				<ReactiveSequence>
					<AlwaysSuccess/>
					<Fallback>
						<AlwaysFailure/>
						<AlwaysSuccess/>
					</Fallback>
					<AlwaysSuccess/>
				</ReactiveSequence>
				<SequenceWithMemory>
					<AlwaysSuccess/>
					<ReactiveFallback>
						<AlwaysFailure/>
						<AlwaysSuccess/>
					</ReactiveFallback>
					<AlwaysSuccess/>
				</SequenceWithMemory>
				<WhileDoElse>
					<ReactiveSequence>
						<AlwaysSuccess/>
						<AlwaysSuccess/>
						<AlwaysSuccess/>
					</ReactiveSequence>
					<SubTree ID="subtree1" />
					<Fallback>
						<AlwaysFailure/>
						<AlwaysSuccess/>
					</Fallback>
				</WhileDoElse>
			</ParallelAll>
		</Fallback>
	</BehaviorTree>

	<BehaviorTree ID="subtree1">
		<Parallel failure_count="3">
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<Sequence>
				<AlwaysSuccess/>
				<Fallback>
					<AlwaysFailure/>
					<ReactiveSequence>
						<ReactiveFallback>
							<AlwaysFailure/>
							<AlwaysSuccess/>
						</ReactiveFallback>
						<AlwaysFailure/>
					</ReactiveSequence>
					<AlwaysSuccess/>
				</Fallback>
				<AlwaysSuccess/>
			</Sequence>
			<AlwaysSuccess/>
			<AlwaysFailure/>
		</Parallel>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn complex() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, ChangeStateAfter, "AlwaysFailure", BehaviorState::Running, BehaviorState::Failure, 5).expect("snh");
	register_behavior!(factory, ChangeStateAfter, "AlwaysSuccess", BehaviorState::Running, BehaviorState::Success, 5).expect("snh");
	register_behavior!(factory, Fallback, "Fallback").expect("snh");
	register_behavior!(factory, Parallel, "Parallel").expect("snh");
	register_behavior!(factory, ParallelAll, "ParallelAll").expect("snh");
	register_behavior!(factory, ReactiveFallback, "ReactiveFallback").expect("snh");
	register_behavior!(factory, ReactiveSequence, "ReactiveSequence").expect("snh");
	register_behavior!(factory, Sequence, "Sequence").expect("snh");
	register_behavior!(factory, SequenceWithMemory, "SequenceWithMemory").expect("snh");
	register_behavior!(factory, WhileDoElse, "WhileDoElse").expect("snh");

	let mut tree = factory.create_from_text(TREE)?;
	drop(factory);

	let mut result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Failure);
	tree.reset().await.expect("snh");
	result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Failure);
	Ok(())
}
