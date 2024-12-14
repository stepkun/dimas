//! Copyright © 2024 Stephan Kunz

use core::fmt::Debug;
use dimas_core::{
	Activity, ActivityType, OperationState, Operational, OperationalType, Transitions,
};

// #[dimas::activity(attr = "wrong attribute")]
// #[derive(Debug)]
// struct TestActivity {}

// #[dimas::activity]
// const fn test_fn() {}

// #[dimas::activity]
// struct Test();

#[dimas_macros::activity]
#[derive(Debug)]
struct TestActivity1<P>
where
	P: Debug + Send + Sync,
{
	dummy: P,
}

impl<P> Transitions for TestActivity1<P> where P: Debug + Send + Sync {}

#[dimas_macros::activity]
#[derive(Debug, Default)]
struct TestActivity2 {
	dummy: String,
}

impl TestActivity2 {
	fn dummy(&self) -> &str {
		&self.dummy
	}
}

impl Transitions for TestActivity2 {}

#[test]
fn activity() {
	let data = ActivityType::default();
	assert!(data.id().is_empty());

	let activity = TestActivity2::default();
	assert_eq!(activity.dummy(), "");
	assert_eq!(activity.id(), "");
}
