// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of sequence behaviors [`Sequence`], [`ReactiveSequence`] and [`SequenceWithMemory`]

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::ChangeStateAfter,
		control::{ReactiveSequence, Sequence, SequenceWithMemory},
	},
	factory::BehaviorTreeFactory,
	register_behavior,
};

const SEQUENCE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step4"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step5"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

fn sequence(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysFailure",
		BehaviorState::Running,
		BehaviorState::Failure,
		5
	)
	.expect("snh");
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysSuccess",
		BehaviorState::Running,
		BehaviorState::Success,
		5
	)
	.expect("snh");
	register_behavior!(factory, Sequence, "Sequence").expect("snh");

	// create the BT
	let mut tree = factory.create_from_text(SEQUENCE).expect("snh");
	drop(factory);

	c.bench_function("sequence", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.block_on(async {
					tree.reset().await.expect("snh");
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

const REACTIVE_SEQUENCE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveSequence name="root_reactive_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step4"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step5"/>
		</ReactiveSequence>
	</BehaviorTree>
</root>
"#;

fn reactive_sequence(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysFailure",
		BehaviorState::Running,
		BehaviorState::Failure,
		5
	)
	.expect("snh");
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysSuccess",
		BehaviorState::Running,
		BehaviorState::Success,
		5
	)
	.expect("snh");
	register_behavior!(factory, ReactiveSequence, "ReactiveSequence").expect("snh");

	// create the BT
	let mut tree = factory
		.create_from_text(REACTIVE_SEQUENCE)
		.expect("snh");
	drop(factory);

	c.bench_function("reactive sequence", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.block_on(async {
					tree.reset().await.expect("snh");
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

const SEQUENCE_WITH_MEMORY: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<SequenceWithMemory name="root_sequence_with_memory">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step4"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="step5"/>
		</SequenceWithMemory>
	</BehaviorTree>
</root>
"#;

fn sequence_with_memory(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysFailure",
		BehaviorState::Running,
		BehaviorState::Failure,
		5
	)
	.expect("snh");
	register_behavior!(
		factory,
		ChangeStateAfter,
		"AlwaysSuccess",
		BehaviorState::Running,
		BehaviorState::Success,
		5
	)
	.expect("snh");
	register_behavior!(factory, SequenceWithMemory, "SequenceWithMemory").expect("snh");

	// create the BT
	let mut tree = factory
		.create_from_text(SEQUENCE_WITH_MEMORY)
		.expect("snh");
	drop(factory);

	c.bench_function("sequence with memory", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.block_on(async {
					tree.reset().await.expect("snh");
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, sequence, reactive_sequence, sequence_with_memory,);

criterion_main!(benches);
