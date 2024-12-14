// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_core::{
	Activity, ActivityId, ActivityType, Agent, Component, ComponentId, ComponentType,
	OperationState, Operational, OperationalType, TaskSignal,
};

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Agent>();
	is_normal::<ActivityId>();
	is_normal::<Box<dyn Activity>>();
	is_normal::<ActivityType>();
	is_normal::<ComponentId>();
	is_normal::<Box<dyn Component>>();
	is_normal::<ComponentType>();
	is_normal::<Box<dyn Operational>>();
	is_normal::<OperationState>();
	is_normal::<OperationalType>();
	is_normal::<TaskSignal>();
}
