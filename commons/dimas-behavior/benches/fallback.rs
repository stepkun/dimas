// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of Sequence behaviors

#[doc(hidden)]
extern crate alloc;

mod behaviors;

use behaviors::{AlwaysFailure, AlwaysSuccess};
use criterion::{Criterion, criterion_group, criterion_main};
use dimas_behavior::factory::NewBehaviorTreeFactory;

const FALLBACK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Fallback name="root_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

fn fallback(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = NewBehaviorTreeFactory::with_core_behaviors();
	factory.register_node_type::<AlwaysSuccess>("AlwaysSuccess");
	factory.register_node_type::<AlwaysFailure>("AlwaysFailure");

	// create the BT
	let mut tree = factory.create_from_text(FALLBACK).unwrap();

	c.bench_function("fallback", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

const REACTIVE_FALLBACK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<ReactiveFallback name="root_fallback">
			<AlwaysFailure	name="step1"/>
			<AlwaysFailure	name="step2"/>
			<AlwaysFailure	name="step3"/>
			<AlwaysSuccess	name="step4"/>
		</ReactiveFallback>
	</BehaviorTree>
</root>
"#;

fn reactive_fallback(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let mut factory = NewBehaviorTreeFactory::with_core_behaviors();
	factory.register_node_type::<AlwaysSuccess>("AlwaysSuccess");
	factory.register_node_type::<AlwaysFailure>("AlwaysFailure");

	// create the BT
	let mut tree = factory
		.create_from_text(REACTIVE_FALLBACK)
		.unwrap();

	c.bench_function("reactive fallback", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				runtime.block_on(async {
					let _result = tree.tick_while_running().await.unwrap();
				});
			});
		});
	});
}

criterion_group!(benches, fallback, reactive_fallback,);

criterion_main!(benches);
