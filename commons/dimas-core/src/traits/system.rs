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

use super::{Configuration, Connection, Operational, Component};
// endregion:	--- modules

// region:		--- System
/// Contract for a `System`
pub trait System: Debug + Operational {
	/// get iterator for [`Component`]s
	fn components(&self) -> impl Iterator<Item = (usize, &Box<dyn Component>)>;

	/// get all connections
	fn connections(&self) -> Vec<Box<dyn Connection>> {
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

	/// Load a library into [`System`]
	/// # Errors
	/// 
	fn load_library(&mut self, path: &str) -> Result<()>;

	/// Unload a library from [`System`]
	/// # Errors
	/// 
	fn unload_library(&mut self, path: &str) -> Result<()>;

}
// endregion:   --- System
