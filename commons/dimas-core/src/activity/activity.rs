// Copyright © 2024 Stephan Kunz

//! Activity interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use anyhow::Result;
use tracing::{event, instrument, Level};

use crate::{Operational, OperationState};
// endregion:	--- modules

// region:		--- types
/// An identifier for an [`Activity`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ActivityId = String;
// endregion:	--- types

// region:		--- Activity
/// Contract for an `Activity`
pub trait Activity: Operational + Send + Sync {
	/// Get [`Activity`]s id
	fn id(&self) -> ActivityId;

	/// Get [`Activity`]s id
	fn set_id(&mut self, id: ActivityId);

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
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}

		// step down?
		while self.state() > desired_state {
			assert!(self.state() > OperationState::Created);
			let next_state = self.state() - 1;
			// next do own transition
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}
		Ok(())
	}
}
// endregion:	--- Activity
