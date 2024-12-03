// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! [`ActivityType`] implementation for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use core::fmt::Debug;

#[cfg(doc)]
use super::Activity;
use crate::{OperationState, Operational, OperationalType, Transitions};
// endregion:	--- modules

// region:		--- ActivityType
/// Data necessary for an [`Activity`].
#[derive(Clone, Debug)]
pub struct ActivityType {
	id: String,
	operational: OperationalType,
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
// endregion:	--- ActivityType
