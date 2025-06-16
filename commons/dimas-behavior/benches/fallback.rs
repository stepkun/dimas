// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of Fallback behaviors [`Fallback`] and [`ReactiveFallback`]

#[doc(hidden)]
extern crate alloc;

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::{
	behavior::{
		BehaviorState, BehaviorStatic,
		action::StateAfter,
		control::{fallback::Fallback, reactive_fallback::ReactiveFallback},
	},
	factory::BehaviorTreeFactory,
	register_node,
};

const FALLBACK: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Fallback name="root_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step3"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step4"/>
			<AlwaysFailure/>
			<AlwaysSuccess	name="step5"/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

fn fallback(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 5).expect("snh");
	register_node!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	register_node!(factory, Fallback, "Fallback").expect("snh");

	// create the BT
	let mut tree = factory.create_from_text(FALLBACK).expect("snh");
	drop(factory);

	c.bench_function("fallback", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				tree.reset().expect("snh");
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

const REACTIVE_FALLBACK: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveFallback name="root_reactive_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step3"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step4"/>
			<AlwaysFailure/>
			<AlwaysFailure	name="step5"/>
		</ReactiveFallback>
	</BehaviorTree>
</root>
"#;

fn reactive_fallback(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.expect("snh");

	let mut factory = BehaviorTreeFactory::default();
	register_node!(factory, StateAfter, "AlwaysFailure", BehaviorState::Failure, 5).expect("snh");
	register_node!(factory, StateAfter, "AlwaysSuccess", BehaviorState::Success, 5).expect("snh");
	register_node!(factory, ReactiveFallback, "ReactiveFallback").expect("snh");

	// create the BT
	let mut tree = factory
		.create_from_text(REACTIVE_FALLBACK)
		.expect("snh");
	drop(factory);

	c.bench_function("reactive fallback", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				tree.reset().expect("snh");
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.expect("snh");
				});
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, fallback, reactive_fallback,);

criterion_main!(benches);
