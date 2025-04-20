// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]
#![allow(clippy::unit_arg)]
#![allow(clippy::unwrap_used)]

//! Benchmarks of scripting equality

use criterion::{Criterion, criterion_group, criterion_main};
use dimas_scripting::{DefaultEnvironment, Parser, VM};

fn boolean_equality(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("true==true; true==false; false==true; false==false;")
		.unwrap();

	c.bench_function("boolean equality", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn double_equality(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("1==1; 3.1475==4.99999; -3.00987654321234==-3.00987654321234; 3.00987654321234==4;")
		.unwrap();

	c.bench_function("double equality", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn integer_equality(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("0x1==0x1; 0xFF321==0x56adf; -0x34==-0x34; 0xabcdef==0x1;")
		.unwrap();

	c.bench_function("integer equality", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn string_equality(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();
	let mut parser = Parser::new();
	let chunk = parser.parse("'short'=='short'; 'short'=='sho'; 'medium'=='this is a little bit longer'; 'this is a little bit longer'=='this is a little bit longer';").unwrap();

	c.bench_function("string equality", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
		});
	});
}

fn mixed_equality(c: &mut Criterion) {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("'short'==true; 'short'==1; 'medium'==nil; 'this is a little bit longer'==0x15;")
		.unwrap();

	c.bench_function("mixed equality", |b| {
		b.iter(|| {
			std::hint::black_box(for _ in 1..=100 {
				vm.run(&chunk, &env, &mut stdout).unwrap();
			});
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
