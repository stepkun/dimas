// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, vec::Vec};
use anyhow::Result;
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::{Activity, ActivityId, Agent, ManageOperationState, OperationState};

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`Component`].
#[derive(Debug, Default)]
pub struct ComponentType {
	id: ComponentId,
	activities: Vec<Box<dyn Activity>>,
	components: Vec<Box<dyn Component>>,
	agent: Option<Agent>,
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
	fn uuid(&self) -> Uuid {
		Uuid::new_v4()
	}

	#[inline]
	fn id(&self) -> ComponentId {
		self.id.clone()
	}

	#[inline]
	fn version(&self) -> u32 {
		0
	}

	#[inline]
	fn set_agent(&mut self, agent: Agent) {
		self.agent = Some(agent);
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
	fn add_component(&mut self, component: Box<dyn Component>) {
		self.components.push(component);
	}

	#[inline]
	fn remove_component(&mut self, _id: ComponentId) {
		todo!()
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
			agent: None,
		}
	}
}
// endregion:	--- ComponentType
