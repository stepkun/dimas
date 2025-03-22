// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting template

use dimas_core::scripting::{Parser, VM};

#[test]
#[ignore = "Template"]
fn template() {
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
}
