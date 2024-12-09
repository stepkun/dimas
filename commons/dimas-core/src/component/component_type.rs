// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use tracing::{event, instrument, Level};

use crate::{Activity, ActivityId, ManageOperationState, OperationState};

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`Component`].
#[derive(Default)]
pub struct ComponentType {
	id: ComponentId,
	activities: Vec<Box<dyn Activity>>,
	components: Vec<Box<dyn Component>>,
}

impl ManageOperationState for ComponentType {
	#[instrument(level = Level::DEBUG, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "manage_operation_state");
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
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
		self.activities.push(activity);
	}

	#[inline]
	fn remove_activity(&mut self, _id: ActivityId) {
		todo!()
	}

	#[inline]
	fn activities(&self) -> &Vec<Box<dyn Activity>> {
		&self.activities
	}

	#[inline]
	fn activities_mut(&mut self) -> &mut Vec<Box<dyn Activity>> {
		&mut self.activities
	}
	#[inline]

	fn add_component(&mut self, component: Box<dyn Component>) {
		self.components.push(component);
	}

	#[inline]
	fn remove_component(&mut self, _id: ComponentId) {
		todo!()
	}

	#[inline]
	fn components(&self) -> &Vec<Box<dyn Component>> {
		&self.components
	}

	#[inline]
	fn components_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
		&mut self.components
	}
}

impl ComponentType {
	/// Create a [`ComponentType`] with given id.
	#[must_use]
	pub fn new(id: ComponentId) -> Self {
		Self {
			id,
			activities: Vec::default(),
			components: Vec::default(),
		}
	}
}
// endregion:	--- ComponentType
