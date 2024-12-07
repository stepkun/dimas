// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::needless_pass_by_value)]

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{
	operational::Transitions, Activity, ActivityId, Component, ComponentId, OperationState,
	Operational, OperationalType,
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{System, SystemId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`System`].
#[derive(Default)]
pub struct SystemType {
	id: SystemId,
}

impl System for SystemType {
	#[inline]
	fn id(&self) -> ComponentId {
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
// endregion:	--- ComponentType
