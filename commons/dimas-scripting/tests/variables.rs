// Copyright © 2025 Stephan Kunz
#![allow(clippy::unwrap_used)]

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn defining_globals() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("test:=3;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");

	stdout.clear();
	let mut parser = Parser::new("@test:=17;print @test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"17\n");

	stdout.clear();
	let mut parser = Parser::new("_test:='string';print _test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"string\n");

	stdout.clear();
	let mut parser = Parser::new("test:=0xf;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"15\n");

	stdout.clear();
	let mut parser = Parser::new("test:='string';print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"string\n");
}

#[test]
fn change_globals() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("test:=3;test=7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"7\n");

	stdout.clear();
	let mut parser = Parser::new("test:=0xf;test=0x1print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"1\n");

	stdout.clear();
	let mut parser = Parser::new("test:='string';test='other';print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"other\n");
}

#[test]
fn assignment_with_change() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("test:=3;test+=7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"10\n");

	stdout.clear();
	let mut parser = Parser::new("test:=0xf;test+=0x1print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"16\n");

	stdout.clear();
	let mut parser = Parser::new("test:='string';test+=' other';print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"string other\n");

	stdout.clear();
	let mut parser = Parser::new("test:=3;test-=7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"-4\n");

	stdout.clear();
	let mut parser = Parser::new("test:=3;test*=7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"21\n");

	stdout.clear();
	let mut parser = Parser::new("test:=6;test/=2;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");
}

#[test]
fn assignment_with_complex_change() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new("test:=3;test+=(17-10)*2-7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"10\n");

	stdout.clear();
	let mut parser = Parser::new("test:=3;test-=(17-10)*2-7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"-4\n");

	stdout.clear();
	let mut parser = Parser::new("test:=3;test*=(17-10)*2-7;print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"21\n");

	stdout.clear();
	let mut parser = Parser::new("test:=6;test/=(17-10)*2-(7+5);print test;");
	let mut chunk = parser.parse().unwrap();
	vm.run(&mut chunk, &env, &mut stdout).unwrap();
	assert_eq!(stdout, b"3\n");
}
