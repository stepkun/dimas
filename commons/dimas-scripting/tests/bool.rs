// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting equality

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn equality() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print true == true;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true == false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == true;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true == 1;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == 0;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true == 'true';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == 'false';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == '';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == '';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true != false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != true;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true != 1;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != 0;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true != 'true';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != 'false';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != '';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn not() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print !true;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !!true;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !!false;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !123;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !0;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !nil;");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !'';");
	let chunk = parser.parse().unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
