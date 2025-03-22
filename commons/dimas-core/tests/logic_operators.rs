// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting template

use dimas_core::scripting::{Parser, VM};

#[test]
#[ignore = "has issues"]
fn and() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print false && 1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	let mut parser = Parser::new("print 1 && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true && 1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print 1 && true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print 1 && 2 && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print 1 && 2 && 3;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}

#[test]
#[ignore = "has issues"]
fn or() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print 1 || true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print false || 1;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}
