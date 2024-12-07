//! Copyright © 2024 Stephan Kunz

use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, OperationState, Operational,
	OperationalType, Transitions,
};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

struct TestComponent {
	operational: OperationalType,
	component: ComponentType,
}

impl AsMut<ComponentType> for TestComponent {
	fn as_mut(&mut self) -> &mut ComponentType {
		&mut self.component
	}
}

impl AsRef<ComponentType> for TestComponent {
	fn as_ref(&self) -> &ComponentType {
		&self.component
	}
}

impl Transitions for TestComponent {}

impl Component for TestComponent {
	#[inline]
	fn id(&self) -> ComponentId {
		self.component.id()
	}

	#[inline]
	fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>> {
		self.component.activities()
	}

	#[inline]
	fn add_activity(&mut self, activity: Box<dyn Activity>) {
		self.component.add_activity(activity);
	}

	#[inline]
	fn remove_activity(&mut self, id: ActivityId) {
		self.component.remove_activity(id);
	}

	#[inline]
	fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>> {
		self.component.activities_mut()
	}

	#[inline]
	fn add_component(&mut self, component: Box<dyn Component>) {
		self.component.add_component(component);
	}

	#[inline]
	fn remove_component(&mut self, id: ComponentId) {
		self.component.remove_component(id);
	}

	#[inline]
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
		self.component.components()
	}

	#[inline]
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
		self.component.components_mut()
	}

	#[inline]
	fn set_id(&mut self, id: String) {
		self.component.set_id(id);
	}
}

impl Operational for TestComponent {
	#[inline]
	fn activation_state(&self) -> OperationState {
		self.operational.activation_state()
	}

	#[inline]
	fn set_activation_state(&mut self, state: OperationState) {
		self.operational.set_activation_state(state);
	}

	#[inline]
	fn desired_state(&self, state: OperationState) -> OperationState {
		self.operational.desired_state(state)
	}

	#[inline]
	fn state(&self) -> OperationState {
		self.operational.state()
	}

	#[inline]
	fn set_state(&mut self, state: OperationState) {
		self.operational.set_state(state);
	}
}

fn create_test_data() -> TestComponent {
	let mut component = TestComponent {
		operational: OperationalType::default(),
		component: ComponentType::new("component".into()),
	};

	let mut component1 = TestComponent {
		operational: OperationalType::with_activation_state(OperationState::Standby),
		component: ComponentType::new("component1".into()),
	};

	let mut component2 = TestComponent {
		operational: OperationalType::with_activation_state(OperationState::Inactive),
		component: ComponentType::new("component2".into()),
	};

	let component3 = TestComponent {
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

#[test]
fn component_type() {
	let _ = ComponentType::new(ComponentId::from("test"));
	let _ = create_test_data();
}
