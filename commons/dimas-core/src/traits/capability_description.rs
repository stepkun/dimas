// Copyright © 2024 Stephan Kunz

//! Capability description interface for `DiMAS` capabilities
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::fmt::Debug;

use super::Capability;
// endregion:	--- modules

// region:		--- CapabilityDescription
/// Contract for a `CapabilityDescription`
pub trait CapabilityDescription: Debug {
	/// get description
	/// # Errors
	/// implementation must fail if no capability is connected
	fn describes(&self) -> Result<Box<dyn Capability>>;
}
// endregion:   --- CapabilityDescription
