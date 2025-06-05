// Copyright © 2025 Stephan Kunz

//! Tests of scripting logic operators

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
fn and() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print false && false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true && false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true && true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false && true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true && true && false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true && true && true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print true || true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false || true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn and_or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print true || true && false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false || true && true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn bitwise_and() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 0x1 & 0x1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime
		.run("print 0x1 & 0x0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"0\n");
}

#[test]
fn bitwise_or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 0x1 | 0x1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime
		.run("print 0x1 | 0x0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime
		.run("print 0x1 | 0x2;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn bitwise_xor() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 0x1 ^ 0x1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"0\n");

	runtime
		.run("print 0x1 ^ 0x0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime
		.run("print 0x1 ^ 0x2;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn ternary() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 1 < 2 ? true : false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print 1 > 2 ? true : false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");
}
