// Copyright © 2025 Stephan Kunz

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Runtime};

use rstest::rstest;

#[rstest]
#[case("print 123.0 + 456.0;", b"579\n")]
#[case("print 123 + 456;", b"579\n")]
#[case("print 'str' + 'ing';", b"string\n")]
#[case("print 'is ' + true;", b"is true\n")]
#[case("print 'is ' + false;", b"is false\n")]
#[case("print 'value is ' + 123;", b"value is 123\n")]
#[case("print 'value is ' + 0xff;", b"value is 255\n")]
#[case("print 'value is ' + nil;", b"value is nil\n")]
fn add(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 4.56 - 1.23;", b"3.3299999999999996\n")]
#[case("print 456 - 123;", b"333\n")]
#[case("print 1.23 - 3.21;", b"-1.98\n")]
fn subtract(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 123 * 456;", b"56088\n")]
#[case("print -123 * 456;", b"-56088\n")]
#[case("print 123 * -456;", b"-56088\n")]
#[case("print 123.0 * 456.0;", b"56088\n")]
#[case("print 1.2 * 3.4;", b"4.08\n")]
#[case("print -1.2 * 3.4;", b"-4.08\n")]
#[case("print 1.2 * -3.4;", b"-4.08\n")]
fn multiply(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 6 / 3;", b"2\n")]
#[case("print -6 / 3;", b"-2\n")]
#[case("print 6 / -3;", b"-2\n")]
#[case("print 6 / 3-6;", b"-4\n")]
#[case("print 6 / (3-6);", b"-2\n")]
#[case("print 1 / 3;", b"0\n")]
#[case("print 1.0 / 3.0;", b"0.3333333333333333\n")]
fn divide(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print (1+2)*3/1+1;", b"10\n")]
#[case("print 1+4*3/6+1;", b"4\n")]
#[case("print (1.1+1.9)*3.3/1.1+1.5;", b"10.499999999999998\n")]
fn precedence(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print nil == nil;", b"true\n")]
#[case("print 0 == 0;", b"true\n")]
#[case("print -0 == 0;", b"true\n")]
#[case("print 0 == -0;", b"true\n")]
#[case("print 1 == 1;", b"true\n")]
#[case("print 1 == 2;", b"false\n")]
#[case("print 'str' == 'str';", b"true\n")]
#[case("print 'str' == 'ing';", b"false\n")]
#[case("print 0 == '0';", b"false\n")]
#[case("print 5.0 == 4.999999999999998;", b"true\n")]
#[case("print 5 == 4.999999999999998;", b"true\n")]
#[case("print 5 == 5.0;", b"true\n")]
#[case("print 5.0 == 4;", b"false\n")]
#[case("print 5 == 4.0;", b"false\n")]
fn equality(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print nil != nil;", b"false\n")]
#[case("print 0 != 0;", b"false\n")]
#[case("print 0 != -0;", b"false\n")]
#[case("print 1 != 1;", b"false\n")]
#[case("print 1 != 2;", b"true\n")]
#[case("print 'str' != 'str';", b"false\n")]
#[case("print 'str' != 'ing';", b"true\n")]
#[case("print 0 != '0';", b"true\n")]
#[case("print 5.0 != 4.999999999999998;", b"false\n")]
#[case("print 5 != 4.999999999999998;", b"false\n")]
#[case("print 5 != 5.0;", b"false\n")]
#[case("print 5.0 != 4;", b"true\n")]
#[case("print 5 != 4.0;", b"true\n")]
fn inequality(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print 1<2;", b"true\n")]
#[case("print 2<2;", b"false\n")]
#[case("print 2<1;", b"false\n")]
#[case("print 1<=2;", b"true\n")]
#[case("print 2<=2;", b"true\n")]
#[case("print 2<=1;", b"false\n")]
#[case("print 1>2;", b"false\n")]
#[case("print 2>2;", b"false\n")]
#[case("print 2>1;", b"true\n")]
#[case("print 1>=2;", b"false\n")]
#[case("print 2>=2;", b"true\n")]
#[case("print 2>=1;", b"true\n")]
#[case("print 0<-0;", b"false\n")]
#[case("print -0<0;", b"false\n")]
#[case("print 0>-0;", b"false\n")]
#[case("print -0>0;", b"false\n")]
#[case("print 0<=-0;", b"true\n")]
#[case("print -0<=0;", b"true\n")]
#[case("print 0>=-0;", b"true\n")]
#[case("print -0>=0;", b"true\n")]
fn comparison(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("print -3;", b"-3\n")]
#[case("print --3;", b"3\n")]
#[case("print ---3;", b"-3\n")]
#[case("print -3.0;", b"-3\n")]
#[case("print --3.0;", b"3\n")]
#[case("print ---3.0;", b"-3\n")]
fn negate(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}
