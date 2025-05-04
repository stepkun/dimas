// Copyright © 2025 Stephan Kunz

//! Tests of scripting expressions

use dimas_scripting::{DefaultEnvironment, Parser, VM};

#[test]
fn expressions() {
	let mut env = DefaultEnvironment::default();
	let mut vm = VM::default();
	let mut stdout: Vec<u8> = Vec::new();

	let mut parser = Parser::new();
	let chunk = parser
		.parse("print (5 - (3 - 1)) + -1;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"2\n");

	stdout.clear();
	let chunk = parser
		.parse("print (5 - (3 - 1)) + +1;")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"4\n");

	stdout.clear();
	let chunk = parser
		.parse("print !(5 - 4 > 3 * 2 == !nil);")
		.expect("snh");
	vm.run(&chunk, &mut env, &mut stdout)
		.expect("snh");
	assert_eq!(stdout, b"true\n");
}
