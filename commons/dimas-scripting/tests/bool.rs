// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting equality

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn equality() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print true == true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print true == false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print true == 1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == 0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true == 'true';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == 'false';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == '';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == '';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true != false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false != true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print true != 1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != 0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print true != 'true';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != 'false';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != '';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn not() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print !true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !!true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !!false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !123;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !nil;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !'';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
