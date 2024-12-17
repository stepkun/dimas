// Copyright © 2024 Stephan Kunz

//! [`ActivityData`] implementation for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use core::fmt::Debug;

use crate::{Activity, ActivityId, Agent, OperationState, Operational, OperationalData, Transitions};
// endregion:	--- modules

// region:		--- ActivityData
/// Data necessary for an [`Activity`].
#[derive(Default)]
pub struct ActivityData {
    /// Id of the activity
	pub id: ActivityId,
	/// The context [`Agent`]
	pub ctx: Option<Agent>,
    /// Operational Data
    pub operational: OperationalData,
}

/// Manual implementation due to possible infinite recursion.
/// References to other components might create a loop.
impl Debug for ActivityData {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("ActivityData")
			.field("id", &self.id)
			.field("ctx", &self.ctx.is_some())
			.field("operational", &self.operational)
			.finish_non_exhaustive()
	}
}

impl Activity for ActivityData {
	#[inline]
	fn id(&self) -> ActivityId {
		self.id.clone()
	}
}



impl Operational for ActivityData {
	fn activation_state(&self) -> OperationState {
		self.operational.activation
	}

	fn set_activation_state(&mut self, state: OperationState) {
        self.operational.activation = state;
	}

	fn state(&self) -> OperationState {
		self.operational.current
	}

	fn set_state(&mut self, state: OperationState) {
        self.operational.current = state;
	}
}

impl Transitions for ActivityData {}

impl ActivityData {
	/// Create [`ActivityData`] with default activation state [`OperationState::Active`].
	#[must_use]
	pub fn new(id: &str, ctx: Agent) -> Self {
		Self { 
            id: id.into(),
            ctx: Some(ctx),
            operational: OperationalData::default(),
        }
	}
}
// endregion:	--- ActivityData
