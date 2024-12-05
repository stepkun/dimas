// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{
	operational::Transitions, Activity, ActivityId, OperationState, Operational, OperationalType,
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`Component`].
#[derive(Clone, Debug, Default)]
pub struct ComponentType {
	id: ComponentId,
	operational: OperationalType,
	activities: Arc<RwLock<Vec<Box<dyn Activity>>>>,
	components: Arc<RwLock<Vec<Box<dyn Component>>>>,
}

impl Component for ComponentType {
	#[inline]
	fn id(&self) -> ComponentId {
		self.id.clone()
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

	fn set_id(&mut self, id: String) {
		self.id = id;
	}
}

impl Transitions for ComponentType {}

impl Operational for ComponentType {
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

impl AsRef<OperationalType> for ComponentType {
	fn as_ref(&self) -> &OperationalType {
		&self.operational
	}
}

impl AsMut<OperationalType> for ComponentType {
	fn as_mut(&mut self) -> &mut OperationalType {
		&mut self.operational
	}
}

impl ComponentType {
	/// Create a [`ComponentType`] with default activation state [`OperationState::Active`].
	#[must_use]
	pub fn new(id: ComponentId) -> Self {
		Self::with_activation_state(id, OperationState::Active)
	}

	/// Create a [`ComponentType`] with given activation state.
	#[must_use]
	pub fn with_activation_state(id: ComponentId, activation_state: OperationState) -> Self {
		Self {
			id,
			operational: OperationalType::with_activation_state(activation_state),
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
