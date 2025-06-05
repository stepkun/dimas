// Copyright © 2025 Stephan Kunz

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Runtime};

#[test]
fn defining_globals() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("test:=3;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");

	runtime
		.run("@test:=17;print @test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"17\n");

	runtime
		.run("_test:='string';print _test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"string\n");

	runtime
		.run("test:=0xf;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"15\n");

	runtime
		.run("test:='string';print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"string\n");
}

#[test]
fn change_globals() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("test:=3;test=7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"7\n");

	runtime
		.run("test:=0xf;test=0x1;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"1\n");

	runtime
		.run("test:='string';test='other';print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"other\n");
}

#[test]
fn assignment_with_change() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("test:=3;test+=7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"10\n");

	runtime
		.run("test:=0xf;test+=0x1;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"16\n");

	runtime
		.run("test:='string';test+=' other';print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"string other\n");

	runtime
		.run("test:=3;test-=7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"-4\n");

	runtime
		.run("test:=3;test*=7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"21\n");

	runtime
		.run("test:=6;test/=2;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn assignment_with_complex_change() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run("test:=3;test+=(17-10)*2-7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"10\n");

	runtime
		.run("test:=3;test-=(17-10)*2-7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"-4\n");

	runtime
		.run("test:=3;test*=(17-10)*2-7;print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"21\n");

	runtime
		.run("test:=6;test/=(17-10)*2-(7+5);print test;", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"3\n");
}

#[test]
fn complex_examples() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime
		.run(
			"param_A:=7;param_B:=5;param_B*=2;param_C:=(param_A*3)+param_B;print param_B;print param_C",
			&mut env,
		)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"10\n31\n");

	runtime
		.run(
			"value:=0x7F;val_A:=value&0x0F;val_B:=value|0xF0;print val_A;print val_B",
			&mut env,
		)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"15\n255\n");

	runtime
		.run("val_A:=2;val_B:=(val_A>1)?42:24;print val_B", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"42\n");

	runtime
		.run("val_A:=0;val_B:=(val_A>1)?42:24;print val_B", &mut env)
		.expect("snh");
	assert_eq!(runtime.stdout(), b"24\n");
}
