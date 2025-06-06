// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of Sequence behaviors

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::factory::BehaviorTreeFactory;

const PARALLEL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Parallel name="root_parallel" failure_count="-1" success_count="2">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Parallel>
	</BehaviorTree>
</root>
"#;

fn parallel(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::with_core_behaviors().expect("snh");

	// create the BT
	let mut tree = factory.create_from_text(PARALLEL).expect("snh");

	c.bench_function("parallel", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

const PARALLEL_ALL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ParallelAll name="root_parallel_all">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysSuccess	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

fn parallel_all(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::with_core_behaviors().expect("snh");

	// create the BT
	let mut tree = factory
		.create_from_text(PARALLEL_ALL)
		.expect("snh");

	c.bench_function("parallel all", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, parallel, parallel_all,);

criterion_main!(benches);
