// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{Activity, ActivityId};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`Component`].
#[derive(Clone, Default)]
pub struct ComponentType {
	id: ComponentId,
	activities: Arc<RwLock<Vec<Box<dyn Activity>>>>,
	components: Arc<RwLock<Vec<Box<dyn Component>>>>,
}

impl Component for ComponentType {
	#[inline]
	fn id(&self) -> ComponentId {
		self.id.clone()
	}

	#[inline]
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
}

impl ComponentType {
	/// Create a [`ComponentType`] with given id.
	#[must_use]
	pub fn new(id: ComponentId) -> Self {
		Self {
			id,
			activities: Arc::new(RwLock::new(Vec::default())),
			components: Arc::new(RwLock::new(Vec::default())),
		}
	}
}
// endregion:	--- ComponentType
