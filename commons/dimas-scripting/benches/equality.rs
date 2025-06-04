// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting equality

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Runtime};

fn boolean_equality(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime
		.parse("true==true; true==false; false==true; false==false;")
		.expect("snh");

	c.bench_function("boolean equality", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn double_equality(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime
		.parse("1==1; 3.1475==4.99999; -3.00987654321234==-3.00987654321234; 3.00987654321234==4;")
		.expect("snh");

	c.bench_function("double equality", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn integer_equality(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime
		.parse("0x1==0x1; 0xFF321==0x56adf; -0x34==-0x34; 0xabcdef==0x1;")
		.expect("snh");

	c.bench_function("integer equality", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn string_equality(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let chunk = runtime.parse("'short'=='short'; 'short'=='sho'; 'medium'=='this is a little bit longer'; 'this is a little bit longer'=='this is a little bit longer';").expect("snh");

	c.bench_function("string equality", |b| {
		b.iter(|| {
			for _ in 1..=100 {
				runtime.execute(&chunk, &mut env).expect("snh");
			}
			std::hint::black_box(());
		});
	});
}

fn mixed_equality(c: &mut Criterion) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("'short'==true; 'short'==1; 'medium'==nil; 'this is a little bit longer'==0x15;")
		.expect("snh");

	c.bench_function("mixed equality", |b| {
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
	boolean_equality,
	double_equality,
	integer_equality,
	string_equality,
	mixed_equality,
);

criterion_main!(benches);
