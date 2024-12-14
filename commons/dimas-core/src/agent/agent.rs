// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{boxed::Box, string::String, sync::Arc};
use anyhow::Result;
use core::{
	any::Any,
	fmt::{Debug, Formatter},
	time::Duration,
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "std")]
use tokio::{select, signal};
use tracing::{error, event, info, instrument, warn, Level};
use uuid::Uuid;

use crate::{
	Activity, ActivityId, Component, ComponentId, ComponentStruct, ManageOperationState,
	OperationState, Operational, Transitions,
};

use super::agent_data::AgentData;
// endregion:   --- modules

// region:      --- Agent
/// Dummy Properties
struct DummyProperties {}

/// Agent
#[derive(Clone, Debug)]
pub struct Agent {
	data: Arc<RwLock<AgentData>>,
	structure: Arc<RwLock<ComponentStruct>>,
	properties: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
}

impl Default for Agent {
	fn default() -> Self {
		Self::new(Box::new(DummyProperties {}))
	}
}

impl ManageOperationState for Agent {
	#[instrument(level = Level::TRACE, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::TRACE, "manage_operation_state");
		let desired_state = self.desired_state(state);
		// step up?
		while self.state() < desired_state {
			assert!(self.state() < OperationState::Active);
			let next_state = self.state() + 1;
			// first handle sub elements
			for component in &mut *self.structure.write().components { //component.state_transition(next_state)?;
			}
			for activity in &mut *self.structure.write().activities {
				activity.state_transitions(next_state)?;
			}
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}
		// step down?
		while self.state() > desired_state {
			assert!(self.state() > OperationState::Created);
			let next_state = self.state() - 1;
			// first handle sub elements
			for activity in &mut *self.structure.write().activities {
				activity.state_transitions(next_state)?;
			}
			for component in &mut *self.structure.write().components { //component.state_transition(next_state)?;
			}
			// next do own transition
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}
		Ok(())
	}
}

impl Operational for Agent {
	fn activation_state(&self) -> OperationState {
		self.data.read().operational.activation
	}

	fn set_activation_state(&mut self, state: OperationState) {
		self.data.write().operational.activation = state;
	}

	fn state(&self) -> OperationState {
		self.data.read().operational.current
	}

	fn set_state(&mut self, state: OperationState) {
		self.data.write().operational.current = state;
	}
}

impl Transitions for Agent {}

impl Agent {
	/// Create an Agent with any properties
	#[must_use]
	pub fn new(properties: Box<dyn Any + Send + Sync>) -> Self {
		Self {
			data: Arc::new(RwLock::new(AgentData::default())),
			structure: Arc::new(RwLock::new(ComponentStruct::default())),
			properties: Arc::new(RwLock::new(properties)),
		}
	}

	/// Get [`Agent`]s unique id
	#[inline]
	#[must_use]
	pub fn uuid(&self) -> Uuid {
		self.data.read().uuid
	}

	/// Get [`Agent`]s name
	#[inline]
	#[must_use]
	pub fn name(&self) -> String {
		self.data.read().name.clone()
	}

	/// Set [`Agent`]s name
	#[inline]
	#[must_use]
	pub fn set_name(self, name: &str) -> Self {
		self.data.write().name = name.into();
		self
	}

	/// Get [`Agent`]s prefix
	#[inline]
	#[must_use]
	pub fn prefix(&self) -> String {
		self.data.read().prefix.clone()
	}

	/// Set [`Agent`]s prefix
	#[inline]
	#[must_use]
	pub fn set_prefix(self, prefix: &str) -> Self {
		self.data.write().prefix = prefix.into();
		self
	}

	/// Add an [`Activity`] to the [`Agent`]
	#[inline]
	pub fn add_activity(&self, activity: Box<dyn Activity>) {
		self.structure.write().activities.push(activity);
	}

	/// Remove an [`Activity`] from the [`Agent`]
	#[inline]
	#[allow(clippy::needless_pass_by_value)]
	pub fn remove_activity(&self, _id: ActivityId) {
		todo!()
	}

	/// Add a [`Component`] to the [`Agent`]
	#[inline]
	pub fn add_component(&self, mut component: Box<dyn Component>) {
		component.set_agent(self.clone());
		self.structure.write().components.push(component);
	}

	/// Remove a [`Component`] from the [`Agent`]
	#[inline]
	#[allow(clippy::needless_pass_by_value)]
	pub fn remove_component(&self, _id: ComponentId) {
		todo!()
	}

	/// Read access to [`Agent`]s properties
	#[inline]
	pub fn read(&self) -> RwLockReadGuard<Box<dyn Any + Send + Sync>> {
		self.properties.read()
	}

	/// Write access to [`Agent`]s properties
	#[inline]
	pub fn write(&self) -> RwLockWriteGuard<Box<dyn Any + Send + Sync>> {
		self.properties.write()
	}

	/// Run the agents main loop
	/// # Errors
	///
	#[allow(clippy::never_loop)]
	#[instrument(level = Level::DEBUG, skip_all)]
	pub async fn start(self) -> Result<()> {
		loop {
			// different possibilities that can happen
			select! {
				// shutdown signal "ctrl-c"
				signal = signal::ctrl_c() => {
					match signal {
						Ok(()) => {
							info!("shutdown due to 'ctrl-c'");
							return self.stop();
						}
						Err(err) => {
							error!("Unable to listen for 'Ctrl-C': {err}");
							// we also try to shut down the agent properly
							return self.stop();
						}
					}
				}
			}
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	#[allow(clippy::unused_self)]
	const fn stop(&self) -> Result<()> {
		Ok(())
	}
}
// endregion:   --- Agent
