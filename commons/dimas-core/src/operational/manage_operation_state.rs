// Copyright © 2024 Stephan Kunz

//! Trait for managing operational states of `DiMAS`
//!

use anyhow::Result;

#[cfg(doc)]
use super::Operational;
use super::OperationState;

/// Trait for management of [`OperationState`]
pub trait ManageOperationState {
	/// Check wether state of contained [`Operational`]s is appropriate for the given [`OperationState`].
	/// If not, adjusts components state to needs considering its sub-components.
	/// # Errors
    fn manage_operation_state(&mut self, state: OperationState) -> Result<()>;
}