// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn add() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 123.0 + 456.0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"579\n");

	stdout.clear();
	let chunk = parser.parse("print 123 + 456;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"579\n");

	stdout.clear();
	let chunk = parser.parse("print 'str' + 'ing';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"string\n");

	stdout.clear();
	let chunk = parser.parse("print 'is ' + true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"is true\n");

	stdout.clear();
	let chunk = parser.parse("print 'is ' + false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"is false\n");

	stdout.clear();
	let chunk = parser.parse("print 'value is ' + 123;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"value is 123\n");

	stdout.clear();
	let chunk = parser.parse("print 'value is ' + 0xff;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"value is 255\n");

	stdout.clear();
	let chunk = parser.parse("print 'is ' + nil;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"is nil\n");
}

#[test]
fn subtract() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 4.56 - 1.23;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3.3299999999999996\n");

	stdout.clear();
	let chunk = parser.parse("print 456 - 123;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"333\n");

	stdout.clear();
	let chunk = parser.parse("print 1.23 - 3.21;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"-1.98\n");
}

#[test]
fn multiply() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 123 * 456;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"56088\n");

	stdout.clear();
	let chunk = parser.parse("print 123.0 * 456.0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"56088\n");

	stdout.clear();
	let chunk = parser.parse("print 1.2 * 3.4;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"4.08\n");
}

#[test]
fn divide() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 6 / 3;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"2\n");

	stdout.clear();
	let chunk = parser.parse("print 1/3;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"0\n");

	stdout.clear();
	let chunk = parser.parse("print 1.0/3.0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"0.3333333333333333\n");
}

#[test]
fn equals() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print nil == nil;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print true == true;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print true == false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 1 == 1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 1 == 2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 'str' == 'str';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 'str' == 'ing';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print nil == false;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print false == 0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 0 == '0';").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");
}

#[test]
fn precedence() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print (1+2)*3/1+1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"10\n");

	stdout.clear();
	let chunk = parser.parse("print 1+4*3/6+1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"4\n");

	stdout.clear();
	let chunk = parser
		.parse("print (1.1+1.9)*3.3/1.1+1.5;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"10.499999999999998\n");
}

#[test]
fn equality() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print 5.0 == 4.999999999999998;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser
		.parse("print 5 == 4.999999999999998;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 5 == 5.0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 5.0 == 4;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser
		.parse("print 5 != 4.999999999999998;")
		.unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 5 != 5.0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 5.0 != 4;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn comparison() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 1<2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 2<2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 2<1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 1<=2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 2<=2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 2<=1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 1>2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 2>2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 2>1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 1>=2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 2>=2;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 2>=1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 2!=1;\nprint 2==1;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\nfalse\n");
}

#[test]
fn special_comparison() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print 0<-0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print -0<0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"false\n");

	stdout.clear();
	let chunk = parser.parse("print 0==-0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 0<=-0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print -0<=0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print 0>=-0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");

	stdout.clear();
	let chunk = parser.parse("print -0>=0;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"true\n");
}

#[test]
fn negate() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser.parse("print -3;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"-3\n");

	stdout.clear();
	let chunk = parser.parse("print --3;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");

	stdout.clear();
	let chunk = parser.parse("print ---3;").unwrap();
	vm.run(&chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"-3\n");
}
