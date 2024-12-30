#[test]
fn tests() {
	let t = trybuild::TestCases::new();
	t.pass("tests/main/01-usage.rs");
	t.pass("tests/main/02-usage.rs");
	t.compile_fail("tests/main/03-wrong-usage.rs");
	t.pass("tests/behavior/01-usage.rs");
	t.compile_fail("tests/behavior/02-wrong-usage.rs");
}
