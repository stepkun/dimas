// Copyright © 2025 Stephan Kunz

//! Tests of scripting equality, inequality & negation

use dimas_scripting::{DefaultEnvironment, Runtime};

use rstest::rstest;

#[rstest]
#[case("print true == true;", b"true\n")]
#[case("print true == false;", b"false\n")]
#[case("print false == true;", b"false\n")]
#[case("print false == false;", b"true\n")]
#[case("print true == 1;", b"false\n")]
#[case("print false == 0;", b"false\n")]
#[case("print true == 'true';", b"false\n")]
#[case("print false == 'false';", b"false\n")]
#[case("print true == '';", b"false\n")]
#[case("print false == '';", b"false\n")]
fn equality(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print true != true;", b"false\n")]
#[case("print true != false;", b"true\n")]
#[case("print false != true;", b"true\n")]
#[case("print false != false;", b"false\n")]
#[case("print true != 1;", b"true\n")]
#[case("print false != 0;", b"true\n")]
#[case("print true != 'true';", b"true\n")]
#[case("print false != 'false';", b"true\n")]
#[case("print true != '';", b"true\n")]
#[case("print false != '';", b"true\n")]
fn inequality(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print !true;", b"false\n")]
#[case("print !false;", b"true\n")]
#[case("print !!true;", b"true\n")]
#[case("print !!false;", b"false\n")]
#[case("print !123;", b"false\n")]
#[case("print !0;", b"false\n")]
#[case("print !1;", b"false\n")]
#[case("print !0.0;", b"false\n")]
#[case("print !1.0;", b"false\n")]
#[case("print !nil;", b"true\n")]
#[case("print !'';", b"false\n")]
#[case("print !'string';", b"false\n")]
fn negation(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}
