// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Operational data of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use core::fmt::Debug;

use super::OperationState;
#[cfg(doc)]
use super::Operational;
// endregion:	--- modules

// region:		--- OperationalData
/// Data necessary for beeing [`Operational`].
#[derive(Debug)]
pub struct OperationalData {
	/// Stores the current [`OperationState`]
	pub current: OperationState,
	/// Stores the parents [`OperationState`], at which this item already will be active
	pub activation: OperationState,
}

impl Default for OperationalData {
	#[inline]
	fn default() -> Self {
		Self::new(OperationState::Active)
	}
}

impl OperationalData {
	/// Create [`OperationalData`] with none default activation state
	#[must_use]
	pub fn new(activation: OperationState) -> Self {
		Self {
			current: OperationState::default(),
			activation,
		}
	}
}
// endregeion:  --- OperationalData
