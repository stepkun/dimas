// Copyright © 2024 Stephan Kunz

//! [`ActivityType`] implementation for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
#[cfg(doc)]
use crate::OperationState;
use crate::{Activity, ActivityId, OperationState, Operational, Transitions};
// endregion:	--- modules

// region:		--- ActivityType
/// Data necessary for an [`Activity`].
#[derive(Debug, Default)]
pub struct ActivityType {
	id: ActivityId,
}

impl Activity for ActivityType {
	#[inline]
	fn id(&self) -> ActivityId {
		self.id.clone()
	}
}

impl Operational for ActivityType {
	fn activation_state(&self) -> OperationState {
		todo!()
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}

	fn state(&self) -> OperationState {
		todo!()
	}

	fn set_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl Transitions for ActivityType {}

impl ActivityType {
	/// Create an [`ActivityType`] with default activation state [`OperationState::Active`].
	#[must_use]
	pub const fn new(id: ActivityId) -> Self {
		Self { id }
	}
}
// endregion:	--- ActivityType
