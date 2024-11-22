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

use crate::error::Error;

use super::Capability;
// endregion:	--- modules

// region:		--- CapabilityDescription
/// contract for a `CapabilityDescription`
pub trait CapabilityDescription: Debug {
    /// get description
    /// # Errors
    /// if function is not implemented
    /// if no capability is connected
    fn describes(&self) -> Result<Box<dyn Capability>> {
        let err = Error::NotImplemented.into();
        Err(err)
    }
}
// endregion:   --- CapabilityDescription