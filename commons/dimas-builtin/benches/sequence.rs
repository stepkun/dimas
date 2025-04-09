// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of Sequence behaviors

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::behavior::{BehaviorResult, BehaviorStatus};
use dimas_builtin::factory::BTFactory;
use dimas_macros::{behavior, register_action};

/// SyncAction "AlwaysSuccessr"
#[behavior(SyncAction)]
struct AlwaysSuccess {}

#[behavior(SyncAction)]
impl AlwaysSuccess {
	async fn tick(&mut self) -> BehaviorResult {
		Ok(BehaviorStatus::Success)
	}
}

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

	let mut factory = BTFactory::default();
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory.create_tree_from_xml(SEQUENCE).unwrap();

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
		<Sequence name="root_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

fn reactive_sequence(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = BTFactory::extended();
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory
		.create_tree_from_xml(REACTIVE_SEQUENCE)
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
		<Sequence name="root_sequence">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

fn sequence_with_memory(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = BTFactory::extended();
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory
		.create_tree_from_xml(SEQUENCE_WITH_MEMORY)
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
