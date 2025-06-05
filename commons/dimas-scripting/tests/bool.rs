// Copyright © 2025 Stephan Kunz

//! Tests of scripting equality

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
#[allow(clippy::too_many_lines)]
fn equality() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print true == true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print true == false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print true == 1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == 0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true == 'true';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == 'false';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == '';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print false == '';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true != false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false != true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false != false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print true != 1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false != 0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print true != 'true';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false != 'false';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print false != '';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn not() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print !true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime
		.run("print !false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print !!true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime
		.run("print !!false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.run("print !123;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.run("print !0;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.run("print !nil;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.run("print !'';", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");
}
