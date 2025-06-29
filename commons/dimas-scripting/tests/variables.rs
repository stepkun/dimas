// Copyright © 2025 Stephan Kunz

//! Tests of scripting operators

use dimas_scripting::{DefaultEnvironment, Runtime};

use rstest::rstest;

#[rstest]
#[case("test:=3;print test;", b"3\n")]
#[case("test:=3.0;print test;", b"3\n")]
#[case("@test:=17;print @test;", b"17\n")]
#[case("_test:='string';print _test;", b"string\n")]
#[case("test:=0xf;print test;", b"15\n")]
#[case("test:='string';print test;", b"string\n")]
fn define_globals(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("test:=3;test=7;print test;", b"7\n")]
#[case("test:=3.0;test=7.0;print test;", b"7\n")]
#[case("test:=0xf;test=0x1;print test;", b"1\n")]
#[case("test:='string';test='other';print test;", b"other\n")]
fn change_globals(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("test:=3;test+=7;print test;", b"10\n")]
#[case("test:=0xf;test+=0x1;print test;", b"16\n")]
#[case("test:='string';test+=' other';print test;", b"string other\n")]
#[case("test:=3;test-=7;print test;", b"-4\n")]
#[case("test:=3;test*=7;print test;", b"21\n")]
#[case("test:=6;test/=2;print test;", b"3\n")]
fn assignment_with_change(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case("test:=3;test+=(17-10)*2-7;print test;", b"10\n")]
#[case("test:=3;test-=(17-10)*2-7;print test;", b"-4\n")]
#[case("test:=3;test*=(17-10)*2-7;print test;", b"21\n")]
#[case("test:=6;test/=(17-10)*2-(7+5);print test;", b"3\n")]
#[case("test:=3.0;test+=(17.0-10.0)*2.0-7.0;print test;", b"10\n")]
fn assignment_with_complex_change(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}

#[rstest]
#[case(
	"param_A:=7;param_B:=5;param_B*=2;param_C:=(param_A*3)+param_B;print param_B;print param_C",
	b"10\n31\n"
)]
#[case(
	"value:=0x7F;val_A:=value&0x0F;val_B:=value|0xF0;print val_A;print val_B",
	b"15\n255\n"
)]
#[case("val_A:=2;val_B:=(val_A>1)?42:24;print val_B", b"42\n")]
#[case("val_A:=0;val_B:=(val_A>1)?42:24;print val_B", b"24\n")]
fn complex_examples(#[case] input: &str, #[case] expected: &[u8]) {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	runtime.run(input, &mut env).expect("snh");
	assert_eq!(runtime.stdout(), expected);
}
