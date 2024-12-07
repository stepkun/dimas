// Copyright © 2024 Stephan Kunz

//! Operational data of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use super::{OperationState, Operational, Transitions};
// endregion:	--- modules

// region:		--- OperationalData
/// Data necessary for an [`Operational`].
pub struct OperationalType {
	current: OperationState,
	activation: OperationState,
}

impl Default for OperationalType {
	#[inline]
	fn default() -> Self {
		Self::new(OperationState::Active)
	}
}

impl Transitions for OperationalType {}

impl Operational for OperationalType {
	#[inline]
	fn activation_state(&self) -> OperationState {
		self.activation
	}

	#[inline]
	fn set_activation_state(&mut self, state: OperationState) {
		self.activation = state;
	}

	#[inline]
	fn state(&self) -> OperationState {
		self.current
	}

	#[inline]
	fn set_state(&mut self, state: OperationState) {
		self.current = state;
	}
}

impl OperationalType {
	/// Creates an [`OperationalType`]
	#[must_use]
	pub fn new(activation: OperationState) -> Self {
		Self {
			current: OperationState::default(),
			activation,
		}
	}

	#[must_use]
	/// Creates an [`OperationalType`] with none defailt activation state
	pub fn with_activation_state(activation_state: OperationState) -> Self {
		Self {
			current: OperationState::default(),
			activation: activation_state,
		}
	}
}
// endregeion:  --- OperationalData
