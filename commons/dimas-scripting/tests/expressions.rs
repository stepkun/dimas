// Copyright © 2025 Stephan Kunz

//! Tests of scripting expressions

use dimas_scripting::{DefaultEnvironment, Runtime};

use rstest::rstest;

#[rstest]
#[case("print (5 - (3 - 1)) + -1;", b"2\n")]
#[case("print (5 - (3 - 1)) + +1;", b"4\n")]
#[case("print !(5 - 4 > 3 * 2 == !nil);", b"true\n")]
fn expressions(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}
