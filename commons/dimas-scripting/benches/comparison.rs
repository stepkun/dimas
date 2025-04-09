// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of scripting equality

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Parser, VM};

fn double_comparison(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new(
		"1<1; 3.1475<4.99999; -3.00987654321234>-3.00987654321234; 4>3.00987654321234;",
	);
	let chunk = parser.parse().unwrap();

	c.bench_function("double comparison", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn integer_comparison(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new("0x1<0x1; 0x1<0x2; 0x1>0x1; 0x2>0x1;");
	let chunk = parser.parse().unwrap();

	c.bench_function("integer comparison", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

criterion_group!(benches, double_comparison, integer_comparison,);

criterion_main!(benches);
