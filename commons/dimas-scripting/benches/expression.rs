// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting expressions

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Runtime};

fn simple_expression(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime.parse("(3 + 2) * (4 - 1);").expect("snh");

	c.bench_function("simple expression", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn moderate_expression(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("!(5 - 4 > 3 * 2 == !nil);")
		.expect("snh");

	c.bench_function("moderate expression", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn string_addition(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("'this is a ' + 'test string';")
		.expect("snh");

	c.bench_function("string addition", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(
	benches,
	simple_expression,
	moderate_expression,
	string_addition,
);

criterion_main!(benches);
