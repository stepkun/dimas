// Copyright © 2024 Stephan Kunz

//! Contract for every `DiMAS` agent
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

use super::{Component, Configuration, Connection, Operational, PluginRegistrar};
// endregion:	--- modules

// region:		--- System
/// Contract for a `System`
pub trait System: Debug + Operational + PluginRegistrar {
	/// get all connections
	fn connections(&self) -> Vec<Box<dyn Connection>> {
		Vec::new()
	}

	/// get all components
	fn components(&self) -> Vec<Box<dyn Component>> {
		Vec::new()
	}

	/// get the [`System`]'s configuration
	/// # Errors
	/// if function is not implemented
	/// implementation must fail if there is no configuration set
	fn configuration(&self) -> Result<Box<dyn Configuration>> {
		let err = Error::NotImplemented.into();
		Err(err)
	}
}
// endregion:   --- System
