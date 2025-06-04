// Copyright © 2025 Stephan Kunz

//! Tests of scripting logic operators

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
fn and() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("print false && false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	let chunk = runtime
		.parse("print true && false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	let chunk = runtime.parse("print true && true;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	let chunk = runtime
		.parse("print false && true;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	let chunk = runtime
		.parse("print true && true && false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	let chunk = runtime
		.parse("print true && true && true;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime.parse("print true || true;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	let chunk = runtime
		.parse("print false || true;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn and_or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("print true || true && false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	let chunk = runtime
		.parse("print false || true && true;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn bitwise_and() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime.parse("print 0x1 & 0x1;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime.clear();
	let chunk = runtime.parse("print 0x1 & 0x0;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"0\n");
}

#[test]
fn bitwise_or() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime.parse("print 0x1 | 0x1;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime.clear();
	let chunk = runtime.parse("print 0x1 | 0x0;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime.clear();
	let chunk = runtime.parse("print 0x1 | 0x2;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn bitwise_xor() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime.parse("print 0x1 ^ 0x1;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"0\n");

	runtime.clear();
	let chunk = runtime.parse("print 0x1 ^ 0x0;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime.clear();
	let chunk = runtime.parse("print 0x1 ^ 0x2;").expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn ternary() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("print 1 < 2 ? true : false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	let chunk = runtime
		.parse("print 1 > 2 ? true : false;")
		.expect("snh");
	runtime.execute(&chunk, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");
}
