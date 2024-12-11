// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::needless_pass_by_value)]

//! System interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use anyhow::Result;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::{event, instrument, Level};

use crate::{
	operational::Transitions, Activity, ActivityId, Component, ComponentId, ManageOperationState,
	OperationState, Operational, OperationalType,
};

use super::{System, SystemId};
// endregion:	--- modules

// region:		--- SystemType
/// Data necessary for a [`System`].
#[derive(Debug, Default)]
pub struct SystemType {
	id: SystemId,
}

impl ManageOperationState for SystemType {
	#[instrument(level = Level::DEBUG, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "manage_operation_state");
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}

impl System for SystemType {
	#[inline]
	fn id(&self) -> SystemId {
		self.id.clone()
	}

	fn set_id(&mut self, id: String) {
		self.id = id;
	}
}

impl SystemType {
	/// Create a [`SystemType`]
	/// Activation state  is always [`OperationState::Active`].
	#[must_use]
	pub const fn new(id: SystemId) -> Self {
		Self { id }
	}
}
// endregion:	--- SystemType
