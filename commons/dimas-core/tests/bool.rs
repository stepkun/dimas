// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting equality

use dimas_core::scripting::{Parser, VM};

#[test]
fn equality() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print true == true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true == false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true == 1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == 0;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true == 'true';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == 'false';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == '';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false == '';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true != false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true != 1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != 0;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print true != 'true';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != 'false';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false != '';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
#[ignore = "Token ! does not work correct"]
fn not() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print !true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !!true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !!false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !123;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !0;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print !nil;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print !'';");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
