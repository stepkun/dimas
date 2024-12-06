// Copyright © 2024 Stephan Kunz

//! [`ActivityType`] implementation for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use core::fmt::Debug;

use crate::{Activity, ActivityId, OperationState, Operational, OperationalType, Transitions};
// endregion:	--- modules

// region:		--- ActivityType
/// Data necessary for an [`Activity`].
#[derive(Clone, Debug, Default)]
pub struct ActivityType {
	id: ActivityId,
	operational: OperationalType,
}

impl Activity for ActivityType {
	#[inline]
	fn id(&self) -> ActivityId {
		self.id.clone()
	}

	#[inline]
	fn set_id(&mut self, id: ActivityId) {
		self.id = id;
	}
}

impl Transitions for ActivityType {}

impl Operational for ActivityType {
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

impl AsRef<OperationalType> for ActivityType {
	fn as_ref(&self) -> &OperationalType {
		&self.operational
	}
}

impl AsMut<OperationalType> for ActivityType {
	fn as_mut(&mut self) -> &mut OperationalType {
		&mut self.operational
	}
}

impl ActivityType {
	/// Create an [`ActivityType`] with default activation state [`OperationState::Active`].
	#[must_use]
	pub fn new(id: ActivityId) -> Self {
		Self::with_activation_state(id, OperationState::Active)
	}

	/// Create a [`ActivityType`] with given activation state.
	#[must_use]
	pub fn with_activation_state(id: ActivityId, activation_state: OperationState) -> Self {
		Self {
			id,
			operational: OperationalType::with_activation_state(activation_state),
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
// endregion:	--- ActivityType
