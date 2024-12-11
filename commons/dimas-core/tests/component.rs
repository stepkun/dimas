//! Copyright © 2024 Stephan Kunz

use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, ManageOperationState,
	OperationState, Operational, OperationalType, Transitions,
};
use uuid::Uuid;

#[dimas_macros::component_old]
#[derive(Debug)]
struct TestComponent1<P>
where
	P: Debug + Send + Sync,
{
	dummy: P,
}

impl<P> Transitions for TestComponent1<P> where P: Debug + Send + Sync {}

impl<P> ManageOperationState for TestComponent1<P>
where
	P: Debug + Send + Sync,
{
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

#[dimas_macros::component_old]
#[derive(Debug, Default)]
struct TestComponent2 {}

impl TestComponent2 {}

impl Transitions for TestComponent2 {}

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

fn create_test_data() -> TestComponent2 {
	let mut component = TestComponent2 {
		operational: OperationalType::default(),
		component: ComponentType::new("component".into()),
	};

	let mut component1 = TestComponent2 {
		operational: OperationalType::with_activation_state(OperationState::Standby),
		component: ComponentType::new("component1".into()),
	};

	let mut component2 = TestComponent2 {
		operational: OperationalType::with_activation_state(OperationState::Inactive),
		component: ComponentType::new("component2".into()),
	};

	let component3 = TestComponent2 {
		operational: OperationalType::with_activation_state(OperationState::Created),
		component: ComponentType::new("component3".into()),
	};

	// create structure
	component2
		.component
		.add_component(Box::new(component3));
	component1
		.component
		.add_component(Box::new(component2));
	component
		.component
		.add_component(Box::new(component1));

	component
}

fn component_type() {
	let _ = ComponentType::new(ComponentId::from("test"));
	let _ = create_test_data();
}

#[test]
fn component() {
	component_trait();
	component_type();
}
