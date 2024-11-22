// Copyright © 2024 Stephan Kunz

//! Capability interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::fmt::Debug;

use super::CapabilityDescription;
// endregion:	--- modules

// region:		--- Capability
/// Contract for a `Capability`
pub trait Capability: Debug {
	/// get description
	/// # Errors
	/// implementation must fail if no description is set
	fn description(&self) -> Result<Box<dyn CapabilityDescription>>;
}
// endregion:   --- Capability
