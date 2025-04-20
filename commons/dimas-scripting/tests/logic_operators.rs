// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting logic operators

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn and() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print false && false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true && false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true && true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false && true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print true && true && false;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print true && true && true;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn or() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print true || true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false || true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn and_or() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print true || true && false;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser
		.parse("print false || true && true;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn bitwise_and() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 & 0x1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 & 0x0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"0\n");
}

#[test]
fn bitwise_or() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 | 0x1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 | 0x0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 | 0x2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");
}

#[test]
fn bitwise_xor() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0x1 ^ 0x1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"0\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 ^ 0x0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let chunk = parser.parse("print 0x1 ^ 0x2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");
}

#[test]
fn ternary() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print 1 < 2 ? true : false;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser
		.parse("print 1 > 2 ? true : false;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
