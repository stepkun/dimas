// Copyright © 2025 Stephan Kunz

//! Tests of scripting equality

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn equality() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print true == true;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print true == false;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == true;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print false == false;")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print true == 1;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == 0;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print true == 'true';")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print false == 'false';")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == '';").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == '';").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print true != false;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print false != true;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("print false != false;")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print true != 1;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != 0;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("print true != 'true';")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser
		.parse("print false != 'false';")
		.expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new();
	let chunk = parser.parse("print false != '';").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");
}

#[test]
fn not() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print !true;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !false;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !!true;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !!false;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !123;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !0;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print !nil;").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print !'';").expect("snh");
	vm.run(&chunk, &env, &mut stdout).expect("snh");
	assert_eq!(stdout, b"false\n");
}
