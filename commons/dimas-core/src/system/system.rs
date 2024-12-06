// Copyright © 2024 Stephan Kunz

//! Contract for every `DiMAS` agent
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use tracing::{event, instrument, Level};

#[cfg(doc)]
use crate::Component;
use crate::{error::Error, Activity, ActivityId, Component, ComponentId, Configuration, Connection, OperationState, Operational};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`System`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type SystemId = String;
// endregion:	--- types

// region:		--- System
/// Contract for a `System`
pub trait System: Operational + Send + Sync {
	/// Get the [`System`]s unique ID
	fn id(&self) -> SystemId;

	/// Set the [`System`]s unique ID
	fn set_id(&mut self, id: SystemId);

	/// Add a sub [`Activity`]
	fn add_activity(&mut self, activity: Box<dyn Activity>);

	/// Remove the sub [`Activity`] with the given `id`
	fn remove_activity(&mut self, id: ActivityId);

	/// Read access to activities
	/// @TODO: should return an Iterator
	fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>>;

	/// Write access to activities
	/// @TODO: should return an Iterator
	fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>>;

	/// Add a sub [`Component`]
	fn add_component(&mut self, component: Box<dyn Component>);

	/// Remove the sub [`Component`] with the given `id`
	fn remove_component(&mut self, id: ComponentId);

	/// Read access to sub components
	/// @TODO: should return an Iterator
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>>;

	/// Write access to sub components
	/// @TODO: should return an Iterator
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>>;
	
	// /// get all connections
	// fn connections(&self) -> Vec<Box<dyn Connection>> {
	// 	Vec::new()
	// }

	// /// get the [`System`]'s configuration
	// /// # Errors
	// /// if function is not implemented
	// /// implementation must fail if there is no configuration set
	// fn configuration(&self) -> Result<Box<dyn Configuration>> {
	// 	let err = Error::NotImplemented.into();
	// 	Err(err)
	// }

	// /// Load a library into [`System`]
	// /// # Errors
	// ///
	// fn load_library(&mut self, path: &str) -> Result<()>;

	// /// Unload a library from [`System`]
	// /// # Errors
	// ///
	// fn unload_library(&mut self, path: &str) -> Result<()>;

	/// Check wether state of [`Operational`] is appropriate for the given [`OperationState`].
	/// If not, adjusts components state to needs considering its sub-components.
	/// # Errors
	#[instrument(level = Level::TRACE, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::TRACE, "manage_operation_state");
		let desired_state = self.desired_state(state);
		// step up?
		while self.state() < desired_state {
			assert!(self.state() < OperationState::Active);
			let next_state = self.state() + 1;
			// first handle sub elements
			for component in &mut *self.components_mut() {
				component.manage_operation_state(next_state)?;
			}
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}

		// step down?
		while self.state() > desired_state {
			assert!(self.state() > OperationState::Created);
			let next_state = self.state() - 1;
			// first handle sub elements
			for component in &mut *self.components_mut() {
				component.manage_operation_state(next_state)?;
			}
			// next do own transition
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}

		Ok(())
	}
}
// endregion:   --- System
