//! Copyright © 2024 Stephan Kunz

use dimas_core::{Activity, OperationState, Operational, OperationalType, Transitions};

// #[dimas::activity(attr = "wrong attribute")]
// #[derive(Debug)]
// struct TestActivity {}

// #[dimas::activity]
// const fn test_fn() {}

// #[dimas::activity]
// struct Test();

#[dimas_macros::activity]
struct TestActivity {
	dummy: String,
}

impl TestActivity {
	fn dummy(&self) -> &str {
		&self.dummy
	}
}

impl Transitions for TestActivity {}

#[test]
fn activity() {
	let mut activity = TestActivity::default();
	assert_eq!(activity.dummy(), "");
	assert_eq!(activity.id(), "");
	activity.set_id("new id".into());
	assert_eq!(activity.id(), "new id");
}
