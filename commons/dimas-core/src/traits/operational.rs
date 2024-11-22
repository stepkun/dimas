// Copyright © 2024 Stephan Kunz

//! Lifecycle interface for `DiMAS` entities
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::enums::OperationState;
use anyhow::Result;
use core::fmt::Debug;
// endregion:	--- modules

// region:		--- Operational
/// Operational
pub trait Operational: Debug {
	/// Checks whether state of component is appropriate for the given [`OperationState`].
	/// If not, implementation has to adjusts components state to needs.
	/// # Errors
	fn manage_operation_state(&self, state: &OperationState) -> Result<()>;
}
// endregion:	--- Operational
