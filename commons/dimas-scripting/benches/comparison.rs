// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting comparison

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Runtime};

fn double_comparison(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime
		.parse("1<1; 3.1475<4.99999; -3.00987654321234>-3.00987654321234; 4>3.00987654321234;")
		.expect("snh");

	c.bench_function("double comparison", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn integer_comparison(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime
		.parse("0x1<0x1; 0x1<0x2; 0x1>0x1; 0x2>0x1;")
		.expect("snh");

	c.bench_function("integer comparison", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, double_comparison, integer_comparison,);

criterion_main!(benches);
