// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of parallel behaviors [`Parallel`] and [`ParallelAll`]

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::AlwaysAfter,
		control::{parallel::Parallel, parallel_all::ParallelAll},
	},
	factory::BehaviorTreeFactory,
	register_node,
};

const PARALLEL: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Parallel name="root_parallel" failure_count="-1" success_count="25">
			<AlwaysFailure	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysFailure	name="step3"/>
			<AlwaysSuccess	name="step4"/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
		</Parallel>
	</BehaviorTree>
</root>
"#;

fn parallel(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, AlwaysAfter, "AlwaysFailure", BehaviorState::Failure, 5).expect("snh");
	register_node!(factory, AlwaysAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	factory
		.register_node_type::<Parallel>("Parallel")
		.expect("snh");

	// create the BT
	let mut tree = factory.create_from_text(PARALLEL).expect("snh");
	drop(factory);

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

const PARALLEL_ALL: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ParallelAll name="root_parallel_all">
			<AlwaysFailure	name="step1"/>
			<AlwaysSuccess	name="step2"/>
			<AlwaysFailure	name="step3"/>
			<AlwaysSuccess	name="step4"/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
			<AlwaysFailure/>
			<AlwaysSuccess/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

fn parallel_all(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, AlwaysAfter, "AlwaysFailure", BehaviorState::Failure, 5).expect("snh");
	register_node!(factory, AlwaysAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	factory
		.register_node_type::<ParallelAll>("ParallelAll")
		.expect("snh");

	// create the BT
	let mut tree = factory
		.create_from_text(PARALLEL_ALL)
		.expect("snh");
	drop(factory);

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
