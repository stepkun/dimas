// Copyright © 2025 Stephan Kunz

//! Tests of scripting logic operators

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn and() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print false && false;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true && false;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true && true;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false && true;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print true && true && false;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print true && true && true;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");
}

#[test]
fn or() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print true || true;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false || true;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");
}

#[test]
fn and_or() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print true || true && false;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser
		.parse("print false || true && true;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");
}

#[test]
fn bitwise_and() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 & 0x1;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 & 0x0;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"0\n");
}

#[test]
fn bitwise_or() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 | 0x1;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 | 0x0;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 | 0x2;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"3\n");
}

#[test]
fn bitwise_xor() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 ^ 0x1;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"0\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 ^ 0x0;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 ^ 0x2;").expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"3\n");
}

#[test]
fn ternary() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print 1 < 2 ? true : false;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser
		.parse("print 1 > 2 ? true : false;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"false\n");
}
