// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of parallel behaviors [`Parallel`] and [`ParallelAll`]

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::StateAfter,
		control::{Parallel, ParallelAll, Sequence},
	},
	factory::BehaviorTreeFactory,
	register_behavior,
};

const SUBTREE: &str = r#"
<root BTCPP_format="4">
	<BehaviorTree ID="SubSequence">
		<Sequence>
			<AlwaysSuccess	name="sub_step1"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="sub_step2"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="sub_step3"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="sub_step4"/>
			<AlwaysSuccess/>
			<AlwaysSuccess	name="sub_step5"/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

const PARALLEL: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Parallel name="root_parallel" failure_count="-1" success_count="5">
			<SubTree ID="SubSequence" name="step1"/>
			<SubTree ID="SubSequence" name="step2"/>
			<SubTree ID="SubSequence" name="step3"/>
			<SubTree ID="SubSequence" name="step4"/>
			<SubTree ID="SubSequence" name="step5"/>
		</Parallel>
	</BehaviorTree>
</root>
"#;

fn parallel(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	register_behavior!(factory, Parallel, "Parallel").expect("snh");
	register_behavior!(factory, Sequence, "Sequence").expect("snh");
	factory
		.register_behavior_tree_from_text(SUBTREE)
		.expect("snh");

	// create the BT
	let mut tree = factory.create_from_text(PARALLEL).expect("snh");
	drop(factory);

	c.bench_function("parallel", |b| {
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

const PARALLEL_ALL: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ParallelAll name="root_parallel_all">
			<SubTree ID="SubSequence" name="step1"/>
			<SubTree ID="SubSequence" name="step2"/>
			<SubTree ID="SubSequence" name="step3"/>
			<SubTree ID="SubSequence" name="step4"/>
			<SubTree ID="SubSequence" name="step5"/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

fn parallel_all(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	register_behavior!(factory, ParallelAll, "ParallelAll").expect("snh");
	register_behavior!(factory, Sequence, "Sequence").expect("snh");
	factory
		.register_behavior_tree_from_text(SUBTREE)
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
					tree.reset().await.expect("snh");
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, parallel, parallel_all,);

criterion_main!(benches);
