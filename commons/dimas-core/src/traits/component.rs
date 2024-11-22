// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, vec::Vec};
use anyhow::Result;
use core::fmt::Debug;

use crate::error::Error;

use super::{Capability, CapabilityDescription, Configuration, Operational, Plugin};
// endregion:	--- modules

// region:		--- Component
/// contract for a `Component`
pub trait Component: Debug + Operational + Plugin {
    /// get provided capabilities
    fn capabilities(&self) -> Vec<Box<dyn Capability>> {
        Vec::new()
    }

    /// get descriptions for provided capabilities
    fn provided_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>> {
        Vec::new()
    }

    /// get descriptions for required capabilities
    fn required_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>> {
        Vec::new()
    }

    /// get the configuration
    /// # Errors
    /// if function is not implemented
    /// if there is no configuration set
    fn configuration(&self) -> Result<Box<dyn Configuration>> {
        let err = Error::NotImplemented.into();
        Err(err)
    }
}
// endregion:   --- Component