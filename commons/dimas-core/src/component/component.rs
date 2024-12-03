// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use core::fmt::Debug;
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use tracing::{event, instrument, Level};

use crate::{Activity, OperationState, Operational};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`Component`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ComponentId = String;
// endregion:	--- types

// region:		--- Component
/// Contract for a `Component`
pub trait Component: Debug + Operational + Send + Sync {
	/// Add a sub [`Component`]
	fn add(&mut self, component: Box<dyn Component>);

	/// Remove the sub [`Component`] with the given `id`
	fn remove(&mut self, id: ComponentId);

	/// Get the [`Components`] unique ID
	fn id(&self) -> ComponentId;

	/// Set the [`Components`] unique ID
	fn set_id(&mut self, id: String);

	/// Read access to activities
	/// @TODO: should return an Iterator
	fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>>;

	/// Write access to activities
	/// @TODO: should return an Iterator
	fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>>;

	/// Read access to sub components
	/// @TODO: should return an Iterator
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>>;

	/// Write access to sub components
	/// @TODO: should return an Iterator
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>>;

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
// endregion:   --- Component

#[cfg(test)]
mod tests {
	use alloc::boxed::Box;

	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<ComponentId>();
		is_normal::<Box<dyn Component>>();
	}
}
