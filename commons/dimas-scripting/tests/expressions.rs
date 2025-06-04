// Copyright © 2025 Stephan Kunz

//! Tests of scripting expressions

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
fn expressions() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print (5 - (3 - 1)) + -1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"2\n");

	runtime.clear();
	runtime
		.run("print (5 - (3 - 1)) + +1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"4\n");

	runtime.clear();
	runtime
		.run("print !(5 - 4 > 3 * 2 == !nil);", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}
