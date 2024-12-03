//! Copyright © 2024 Stephan Kunz

use dimas_core::{Activity, Component, ComponentId, ComponentType, Operational, OperationState, OperationalType, Transitions};

#[dimas_macros::component]
struct TestComponent {}

impl TestComponent {}

impl Transitions for TestComponent {}

#[test]
fn component() {
    let mut component = TestComponent::default();
    assert_eq!(component.id(), "");
    component.set_id("new id".into());
    assert_eq!(component.id(), "new id");
}
