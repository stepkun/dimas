// Copyright © 2025 Stephan Kunz

//! Tests of scripting logic operators

use dimas_scripting::{DefaultEnvironment, Runtime};

use rstest::rstest;

#[rstest]
#[case("print false && false", b"false\n")]
#[case("print true && false", b"false\n")]
#[case("print false && true", b"false\n")]
#[case("print true && true", b"true\n")]
#[case("print false && false && false", b"false\n")]
#[case("print false && false && true", b"false\n")]
#[case("print true && false && false", b"false\n")]
#[case("print true && false && true", b"false\n")]
#[case("print true && true && true", b"true\n")]
fn and(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print false || false", b"false\n")]
#[case("print true || false", b"true\n")]
#[case("print false || true", b"true\n")]
#[case("print true || true", b"true\n")]
#[case("print false || false || false", b"false\n")]
#[case("print false || false || true", b"true\n")]
#[case("print true || false || false", b"true\n")]
#[case("print true || false || true", b"true\n")]
#[case("print true || true || true", b"true\n")]
fn or(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print true || true && false;", b"true\n")]
#[case("print false || true && true;", b"true\n")]
fn and_or(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 0x1 & 0x1;", b"1\n")]
#[case("print 0x1 & 0x0;", b"0\n")]
fn bitwise_and(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 0x1 | 0x1;", b"1\n")]
#[case("print 0x1 | 0x0;", b"1\n")]
#[case("print 0x1 | 0x2;", b"3\n")]
fn bitwise_or(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 0x1 ^ 0x1;", b"0\n")]
#[case("print 0x1 ^ 0x0;", b"1\n")]
#[case("print 0x1 ^ 0x2;", b"3\n")]
fn bitwise_xor(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 1 < 2 ? true : false;", b"true\n")]
#[case("print 1 > 2 ? true : false;", b"false\n")]
fn ternary(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}
