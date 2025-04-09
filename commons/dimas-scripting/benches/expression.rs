// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of scripting expressions

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Parser, VM};

fn simple_expression(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("(3 + 2) * (4 - 1);");
	let chunk = parser.parse().unwrap();

	c.bench_function("simple expression", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn moderate_expression(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("!(5 - 4 > 3 * 2 == !nil);");
	let chunk = parser.parse().unwrap();

	c.bench_function("moderate expression", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn string_addition(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("'this is a ' + 'test string';");
	let chunk = parser.parse().unwrap();

	c.bench_function("string addition", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
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
