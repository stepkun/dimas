// Copyright © 2024 Stephan Kunz
#![allow(unused_imports)]
//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use core::fmt::Debug;

use super::{Capability, CapabilityDescription, Configuration, Operational};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`Component`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ComponentId = String;
// endregion:	--- types

// region:		--- Component
/// Contract for a `Component`
pub trait Component: Debug + Operational + Send + Sync{
	fn id(&self) -> ComponentId;

	// /// get provided capabilities
	// fn capabilities(&self) -> Vec<Box<dyn Capability>>;

	// /// get descriptions for provided capabilities
	// fn provided_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>>;

	// /// get descriptions for required capabilities
	// fn required_capabilities(&self) -> Vec<Box<dyn CapabilityDescription>>;

	// /// get the configuration
	// /// # Errors
	// /// implementation must fail if there is no configuration set
	// fn configuration(&self) -> Result<Box<dyn Configuration>>;
}
// endregion:   --- Component
