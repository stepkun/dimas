//! Copyright © 2024 Stephan Kunz

use anyhow::Result;
use dimas_core::{
	ComponentType, ManageOperationState, OperationState, Operational, OperationalType, System,
	SystemId, SystemType, Transitions,
};

#[dimas_macros::system]
struct TestSystem1<P>
where
	P: Send + Sync,
{
	dummy: P,
}

impl<P> Transitions for TestSystem1<P> where P: Send + Sync {}

impl<P> ManageOperationState for TestSystem1<P>
where
	P: Send + Sync,
{
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

#[dimas_macros::system]
#[derive(Default)]
struct TestSystem2 {}

impl TestSystem2 {}

impl Transitions for TestSystem2 {}

impl ManageOperationState for TestSystem2 {
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

fn system_trait() {
	let mut system = TestSystem2::default();
	assert_eq!(system.id(), "");
	system.set_id("new id".into());
	assert_eq!(system.id(), "new id");
}

fn system_type() {
	let _ = SystemType::new(SystemId::from("test"));
}

#[test]
fn system() {
	system_trait();
	system_type();
}
