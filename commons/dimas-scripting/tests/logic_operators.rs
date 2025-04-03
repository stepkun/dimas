// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting logic operators

use dimas_scripting::{Parser, VM};

#[test]
fn and() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print false && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true && true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false && true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true && true && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let mut parser = Parser::new("print true && true && true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn or() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print true || true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false || true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn and_or() {
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("print true || true && false;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let mut parser = Parser::new("print false || true && true;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}
