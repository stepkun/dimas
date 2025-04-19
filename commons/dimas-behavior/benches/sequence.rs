// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of Sequence behaviors

#[doc(hidden)]
extern crate alloc;

mod behaviors;

use behaviors::AlwaysSuccess;
use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::factory::NewBehaviorTreeFactory;

const SEQUENCE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

fn sequence(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = NewBehaviorTreeFactory::with_core_behaviors().unwrap();
	factory
		.register_node_type::<AlwaysSuccess>("AlwaysSuccess")
		.unwrap();

	// create the BT
	let mut tree = factory.create_from_text(SEQUENCE).unwrap();

	c.bench_function("sequence", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

const REACTIVE_SEQUENCE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveSequence name="root_reactive_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</ReactiveSequence>
	</BehaviorTree>
</root>
"#;

fn reactive_sequence(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = NewBehaviorTreeFactory::with_core_behaviors().unwrap();
	factory
		.register_node_type::<AlwaysSuccess>("AlwaysSuccess")
		.unwrap();

	// create the BT
	let mut tree = factory
		.create_from_text(REACTIVE_SEQUENCE)
		.unwrap();

	c.bench_function("reactive sequence", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

const SEQUENCE_WITH_MEMORY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<SequenceWithMemory name="root_sequence_with_memory">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</SequenceWithMemory>
	</BehaviorTree>
</root>
"#;

fn sequence_with_memory(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = NewBehaviorTreeFactory::with_core_behaviors().unwrap();
	factory
		.register_node_type::<AlwaysSuccess>("AlwaysSuccess")
		.unwrap();

	// create the BT
	let mut tree = factory
		.create_from_text(SEQUENCE_WITH_MEMORY)
		.unwrap();

	c.bench_function("sequence with memory", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

criterion_group!(benches, sequence, reactive_sequence, sequence_with_memory,);

criterion_main!(benches);
