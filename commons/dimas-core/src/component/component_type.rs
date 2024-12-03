// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{operational::Transitions, Activity, OperationState, Operational, OperationalType};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
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
	fn add(&mut self, component: Box<dyn Component>) {
		self.components.write().push(component);
	}

	#[inline]
	fn remove(&mut self, _id: ComponentId) {
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
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
		self.components.read()
	}

	#[inline]
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
		self.components.write()
	}

	fn set_id(&mut self, id: alloc::string::String) {
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

impl ComponentType {
	/// Create a [`ComponentId`] wirh default activation state [`OperationState::Active`].
	#[must_use]
	pub fn new(id: ComponentId) -> Self {
		Self::with_activation_state(id, OperationState::Active)
	}

	/// Create a [`ComponentId`] with given state.
	#[must_use]
	pub fn with_activation_state(id: ComponentId, activation_state: OperationState) -> Self {
		Self {
			id,
			operational: OperationalType::with_activation_state(activation_state),
			activities: Arc::new(RwLock::new(Vec::default())),
			components: Arc::new(RwLock::new(Vec::default())),
		}
	}
}
// endregion:	--- ComponentType

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<ComponentType>();
	}
}
