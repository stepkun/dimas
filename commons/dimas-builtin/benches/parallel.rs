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

/// SyncAction "AlwaysFailure"
#[behavior(SyncAction)]
struct AlwaysFailure {}

#[behavior(SyncAction)]
impl AlwaysFailure {
	async fn tick(&mut self) -> BehaviorResult {
		Ok(BehaviorStatus::Failure)
	}
}

const PARALLEL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Parallel name="root_parallel" failure_count="-1" success_count="2">
			<AlwaysSuccess	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Parallel>
	</BehaviorTree>
</root>
"#;

fn parallel(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = BTFactory::default();
	register_action!(factory, "AlwaysFailure", AlwaysSuccess);
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory.create_tree_from_xml(PARALLEL).unwrap();

	c.bench_function("parallel", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

const PARALLEL_ALL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ParallelAll name="root_parallel">
			<AlwaysSuccess	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

fn parallel_all(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = BTFactory::extended();
	register_action!(factory, "AlwaysFailure", AlwaysSuccess);
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory
		.create_tree_from_xml(PARALLEL_ALL)
		.unwrap();

	c.bench_function("parallel all", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

criterion_group!(benches, parallel, parallel_all,);

criterion_main!(benches);
