// Copyright © 2025 Stephan Kunz

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
fn add() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 123.0 + 456.0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"579\n");

	runtime.clear();
	runtime
		.run("print 123 + 456;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"579\n");

	runtime.clear();
	runtime
		.run("print 'str' + 'ing';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"string\n");

	runtime.clear();
	runtime
		.run("print 'is ' + true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"is true\n");

	runtime.clear();
	runtime
		.run("print 'is ' + false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"is false\n");

	runtime.clear();
	runtime
		.run("print 'value is ' + 123;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"value is 123\n");

	runtime.clear();
	runtime
		.run("print 'value is ' + 0xff;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"value is 255\n");

	runtime.clear();
	runtime
		.run("print 'is ' + nil;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"is nil\n");
}

#[test]
fn subtract() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 4.56 - 1.23;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3.3299999999999996\n");

	runtime.clear();
	runtime
		.run("print 456 - 123;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"333\n");

	runtime.clear();
	runtime
		.run("print 1.23 - 3.21;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"-1.98\n");
}

#[test]
fn multiply() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 123 * 456;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"56088\n");

	runtime.clear();
	runtime
		.run("print 123.0 * 456.0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"56088\n");

	runtime.clear();
	runtime
		.run("print 1.2 * 3.4;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"4.08\n");
}

#[test]
fn divide() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 6 / 3;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"2\n");

	runtime.clear();
	runtime.run("print 1/3;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"0\n");

	runtime.clear();
	runtime
		.run("print 1.0/3.0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"0.3333333333333333\n");
}

#[test]
fn equals() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print nil == nil;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print true == true;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print true == false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 1 == 1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 1 == 2;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 'str' == 'str';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 'str' == 'ing';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print nil == false;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print false == 0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 0 == '0';", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");
}

#[test]
fn precedence() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print (1+2)*3/1+1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"10\n");

	runtime.clear();
	runtime
		.run("print 1+4*3/6+1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"4\n");

	runtime.clear();
	runtime
		.run("print (1.1+1.9)*3.3/1.1+1.5;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"10.499999999999998\n");
}

#[test]
fn equality() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("print 5.0 == 4.999999999999998;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 5 == 4.999999999999998;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 5 == 5.0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 5.0 == 4;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 5 != 4.999999999999998;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 5 != 5.0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 5.0 != 4;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn comparison() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run("print 1<2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime.run("print 2<2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 2<1;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 1<=2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime.run("print 2<=2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime.run("print 2<=1;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 1>2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 2>2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 2>1;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime.run("print 1>=2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print 2>=2;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime.run("print 2>=1;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 2!=1;\nprint 2==1;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\nfalse\n");
}

#[test]
fn special_comparison() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run("print 0<-0;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime.run("print -0<0;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"false\n");

	runtime.clear();
	runtime
		.run("print 0==-0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 0<=-0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print -0<=0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print 0>=-0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");

	runtime.clear();
	runtime
		.run("print -0>=0;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"true\n");
}

#[test]
fn negate() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run("print -3;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"-3\n");

	runtime.clear();
	runtime.run("print --3;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");

	runtime.clear();
	runtime.run("print ---3;", &mut env).expect("snh");
	assert_eq!(runtime.stdout(), b"-3\n");
}
