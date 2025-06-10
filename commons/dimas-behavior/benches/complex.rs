// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of complex scenario

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{BehaviorState, BehaviorStatic, action::StateAfter, control::fallback::Fallback},
	factory::BehaviorTreeFactory,
	register_node,
};

const TREE: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree1">
	<BehaviorTree ID="MainTree1">
		<Fallback name="root_fallback">
			<AlwaysFailure/>
			<AlwaysSuccess/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

fn complex(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 3).expect("snh");
	register_node!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 3).expect("snh");
	factory
		.register_node_type::<Fallback>("Fallback")
		.expect("snh");

	let mut tree = factory.create_from_text(TREE).expect("snh");
	drop(factory);

	c.bench_function("complex", |b| {
		b.iter(|| {
			for _ in 1..=50 {
				tree.reset().expect("snh");
				runtime.block_on(async {
					tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, complex,);

criterion_main!(benches);
