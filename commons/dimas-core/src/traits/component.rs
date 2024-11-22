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

use super::{Capability, CapabilityDescription, Configuration, Plugin};
// endregion:	--- modules

// region:		--- Component
/// Contract for a `Component`
pub trait Component: Debug + Plugin {
	/// get provided capabilities
	fn capabilities(&self) -> Vec<Box<dyn Capability>>;

	/// get descriptions for provided capabilities
	fn provided_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>>;

	/// get descriptions for required capabilities
	fn required_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>>;

	/// get the configuration
	/// # Errors
	/// implementation must fail if there is no configuration set
	fn configuration(&self) -> Result<Box<dyn Configuration>>;
}
// endregion:   --- Component
