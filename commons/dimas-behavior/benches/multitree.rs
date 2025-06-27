// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of multitree scenario

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::StateAfter,
		control::{Fallback, Parallel, ParallelAll, ReactiveFallback, ReactiveSequence, Sequence},
	},
	factory::BehaviorTreeFactory,
	register_behavior,
};
use tokio::join;

const TREE1: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree1">
	<BehaviorTree ID="MainTree1">
		<Sequence name="root_sequence">
			<AlwaysFailure/>
			<SubTree ID="subtree"/>
			<AlwaysSuccess/>
		</Sequence>
	</BehaviorTree>
</root>
"#;

const TREE2: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree2">
	<BehaviorTree ID="MainTree2">
		<Fallback name="root_fallback">
			<AlwaysFailure/>
			<SubTree ID="subtree"/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

const TREE3: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree3">
	<BehaviorTree ID="MainTree3">
		<ParallelAll name="root_parallel">
			<AlwaysFailure/>
			<SubTree ID="subtree"/>
			<AlwaysSuccess/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

const SUBTREE: &str = r#"
<root BTCPP_format="4">
	<BehaviorTree ID="subtree">
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

fn multitree(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_behavior!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 5).expect("snh");
	register_behavior!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	register_behavior!(factory, Fallback, "Fallback").expect("snh");
	register_behavior!(factory, Parallel, "Parallel").expect("snh");
	register_behavior!(factory, ParallelAll, "ParallelAll").expect("snh");
	register_behavior!(factory, ReactiveFallback, "ReactiveFallback").expect("snh");
	register_behavior!(factory, ReactiveSequence, "ReactiveSequence").expect("snh");
	register_behavior!(factory, Sequence, "Sequence").expect("snh");
	factory
		.register_behavior_tree_from_text(SUBTREE)
		.expect("snh");

	let mut tree1 = factory.create_from_text(TREE1).expect("snh");
	let mut tree2 = factory.create_from_text(TREE2).expect("snh");
	let mut tree3 = factory.create_from_text(TREE3).expect("snh");
	drop(factory);

	c.bench_function("multitree", |b| {
		b.iter(|| {
			runtime.block_on(async {
				for _ in 1..=100 {
					let h1 = tree1.reset();
					let h2 = tree2.reset();
					let h3 = tree3.reset();
					let (res1, res2, res3) = join!(h1, h2, h3);
					res1.expect("snh");
					res2.expect("snh");
					res3.expect("snh");
					let h1 = tree1.tick_while_running();
					let h2 = tree2.tick_while_running();
					let h3 = tree3.tick_while_running();
					let (res1, res2, res3) = join!(h1, h2, h3);
					res1.expect("snh");
					res2.expect("snh");
					res3.expect("snh");
				}
			});
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, multitree,);

criterion_main!(benches);
