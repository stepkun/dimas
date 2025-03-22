// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting expressions

use dimas_core::scripting::{Parser, VM};

#[test]
#[ignore = "Token ! does not work correct"]
fn template() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print (5 - (3 - 1)) + -1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"2\n");

	stdout.clear();
	let mut parser = Parser::new("print (5 - (3 - 1)) + +1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"4\n");

	stdout.clear();
	let mut parser = Parser::new("print !(5 - 4 > 3 * 2 == !nil);");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
