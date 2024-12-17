//! Copyright © 2024 Stephan Kunz

use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, ManageOperationState,
	OperationState, Operational, Transitions,
};
use uuid::Uuid;

#[dimas_macros::component]
#[derive(Debug)]
struct TestComponent1<P>
where
	P: Debug + Send + Sync,
{
	dummy: P,
}

impl<P> Transitions for TestComponent1<P> where P: Debug + Send + Sync {}

impl<P> Operational for TestComponent1<P>
where
	P: Debug + Send + Sync,
{
	fn activation_state(&self) -> OperationState {
		todo!()
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}

	fn state(&self) -> OperationState {
		todo!()
	}

	fn set_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl<P> ManageOperationState for TestComponent1<P>
where
	P: Debug + Send + Sync,
{
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

#[dimas_macros::component]
#[derive(Debug, Default)]
struct TestComponent2 {}

impl TestComponent2 {}

impl Transitions for TestComponent2 {}

impl Operational for TestComponent2 {
	fn activation_state(&self) -> OperationState {
		todo!()
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}

	fn state(&self) -> OperationState {
		todo!()
	}

	fn set_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl ManageOperationState for TestComponent2 {
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

fn component_trait() {
	let component = TestComponent2::default();
	assert_eq!(component.id(), "");
}

fn component_type() {
	let _ = ComponentType::new(ComponentId::from("test"));
}

#[test]
fn component() {
	component_trait();
	component_type();
}
