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

	let mut factory = BTFactory::default();
	register_action!(factory, "AlwaysFailure", AlwaysSuccess);
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory.create_tree_from_xml(FALLBACK).unwrap();

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

	let mut factory = BTFactory::extended();
	register_action!(factory, "AlwaysFailure", AlwaysSuccess);
	register_action!(factory, "AlwaysSuccess", AlwaysSuccess);

	// create the BT
	let mut tree = factory
		.create_tree_from_xml(REACTIVE_FALLBACK)
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
