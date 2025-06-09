// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of complex scenarios

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::factory::BehaviorTreeFactory;
use tokio::join;

const TREE1: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree1">
	<BehaviorTree ID="MainTree1">
		<Fallback name="root_fallback">
			<AlwaysSuccess/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

const TREE2: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree2">
	<BehaviorTree ID="MainTree2">
		<Fallback name="root_fallback">
			<AlwaysSuccess/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

const TREE3: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree3">
	<BehaviorTree ID="MainTree3">
		<Fallback name="root_fallback">
			<AlwaysSuccess/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

fn complex(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::with_core_behaviors().expect("snh");

	// create the BT*s
	let mut tree1 = factory.create_from_text(TREE1).expect("snh");
	let mut tree2 = factory.create_from_text(TREE2).expect("snh");
	let mut tree3 = factory.create_from_text(TREE3).expect("snh");

	c.bench_function("complex", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				let h1 = tree1.tick_while_running();
				let h2 = tree2.tick_while_running();
				let h3 = tree3.tick_while_running();
				runtime.block_on(async {
					let _ = join!(h1, h2, h3);
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, complex,);

criterion_main!(benches);
