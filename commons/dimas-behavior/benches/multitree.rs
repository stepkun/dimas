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
		control::{fallback::Fallback, parallel_all::ParallelAll, sequence::Sequence},
	},
	factory::BehaviorTreeFactory,
	register_node,
};
use tokio::join;

const TREE1: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree1">
	<BehaviorTree ID="MainTree1">
		<Sequence name="root_sequence">
			<AlwaysFailure/>
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
			<AlwaysSuccess/>
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
			<AlwaysSuccess/>
		</ParallelAll>
	</BehaviorTree>
</root>
"#;

fn multitree(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 3).expect("snh");
	register_node!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 3).expect("snh");
	factory
		.register_node_type::<Fallback>("Fallback")
		.expect("snh");
	factory
		.register_node_type::<ParallelAll>("ParallelAll")
		.expect("snh");
	factory
		.register_node_type::<Sequence>("Sequence")
		.expect("snh");

	let mut tree1 = factory.create_from_text(TREE1).expect("snh");
	let mut tree2 = factory.create_from_text(TREE2).expect("snh");
	let mut tree3 = factory.create_from_text(TREE3).expect("snh");
	drop(factory);

	c.bench_function("multitree", |b| {
		b.iter(|| {
			for _ in 1..=50 {
				tree1.reset().expect("snh");
				tree2.reset().expect("snh");
				tree3.reset().expect("snh");
				let h1 = tree1.tick_while_running();
				let h2 = tree2.tick_while_running();
				let h3 = tree3.tick_while_running();
				runtime.block_on(async {
					let (res1, res2, res3) = join!(h1, h2, h3);
					res1.expect("snh");
					res2.expect("snh");
					res3.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, multitree,);

criterion_main!(benches);
