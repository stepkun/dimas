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
#[derive(Clone, Debug)]
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

	fn desired_state(&self, state: OperationState) -> OperationState {
		let state: i32 = state.into();
		let state_diff = OperationState::Active - self.activation;

		// limit to bounds [`OperationState::Created`] <=> [`OperationState::Active`]
		let min_state: i32 = OperationState::Created.into();
		let max_state: i32 = OperationState::Active.into();
		let desired_state_int = min_state.max(max_state.min(state + state_diff));

		OperationState::try_from(desired_state_int)
			.unwrap_or_else(|_| panic!("should be infallible"))
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

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<OperationalType>();
	}

	#[test]
	fn operational_type() {
		let data = OperationalType::default();

		assert_eq!(
			data.desired_state(OperationState::Created),
			OperationState::Created
		);
		assert_eq!(
			data.desired_state(OperationState::Inactive),
			OperationState::Inactive
		);
		assert_eq!(
			data.desired_state(OperationState::Active),
			OperationState::Active
		);

		let mut data = OperationalType::new(OperationState::Created);

		assert_eq!(
			data.desired_state(OperationState::Created),
			OperationState::Active
		);
		assert_eq!(
			data.desired_state(OperationState::Inactive),
			OperationState::Active
		);
		assert_eq!(
			data.desired_state(OperationState::Active),
			OperationState::Active
		);

		data.activation = OperationState::Inactive;
		assert_eq!(
			data.desired_state(OperationState::Created),
			OperationState::Inactive
		);
		assert_eq!(
			data.desired_state(OperationState::Inactive),
			OperationState::Active
		);
		assert_eq!(
			data.desired_state(OperationState::Active),
			OperationState::Active
		);
	}
}
