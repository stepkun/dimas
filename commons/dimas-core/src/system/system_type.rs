// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::needless_pass_by_value)]

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{
	operational::Transitions, Activity, ActivityId, Component, ComponentId, OperationState,
	Operational, OperationalType,
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{System, SystemId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`System`].
#[derive(Clone, Default)]
pub struct SystemType {
	id: SystemId,
	operational: OperationalType,
	activities: Arc<RwLock<Vec<Box<dyn Activity>>>>,
	components: Arc<RwLock<Vec<Box<dyn Component>>>>,
}

impl System for SystemType {
	#[inline]
	fn id(&self) -> ComponentId {
		self.id.clone()
	}

	fn set_id(&mut self, id: String) {
		self.id = id;
	}

	#[inline]
	fn add_activity(&mut self, activity: Box<dyn Activity>) {
		self.activities.write().push(activity);
	}

	#[inline]
	fn remove_activity(&mut self, _id: ActivityId) {
		todo!()
	}

	#[inline]
	fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>> {
		self.activities.read()
	}

	#[inline]
	fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>> {
		self.activities.write()
	}
	#[inline]

	fn add_component(&mut self, component: Box<dyn Component>) {
		self.components.write().push(component);
	}

	#[inline]
	fn remove_component(&mut self, _id: ComponentId) {
		todo!()
	}

	#[inline]
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
		self.components.read()
	}

	#[inline]
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
		self.components.write()
	}

	// fn load_library(&mut self, path: &str) -> anyhow::Result<()> {
	// 	todo!()
	// }

	// fn unload_library(&mut self, path: &str) -> anyhow::Result<()> {
	// 	todo!()
	// }
}

impl Transitions for SystemType {}

impl Operational for SystemType {
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

impl AsRef<OperationalType> for SystemType {
	fn as_ref(&self) -> &OperationalType {
		&self.operational
	}
}

impl AsMut<OperationalType> for SystemType {
	fn as_mut(&mut self) -> &mut OperationalType {
		&mut self.operational
	}
}

impl SystemType {
	/// Create a [`SystemType`]
	/// Activation state  is always [`OperationState::Active`].
	#[must_use]
	pub fn new(id: SystemId) -> Self {
		Self {
			id,
			operational: OperationalType::default(),
			activities: Arc::new(RwLock::new(Vec::default())),
			components: Arc::new(RwLock::new(Vec::default())),
		}
	}

	/// Operational
	#[must_use]
	#[inline]
	pub const fn operational(&self) -> &OperationalType {
		&self.operational
	}

	/// Operational mut
	#[must_use]
	#[inline]
	pub fn operational_mut(&mut self) -> &mut OperationalType {
		&mut self.operational
	}
}
// endregion:	--- ComponentType
