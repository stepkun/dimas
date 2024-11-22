// Copyright © 2024 Stephan Kunz

//! Configuration
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
// endregion:	--- modules

// region:		--- Configuration
/// Contract for a `Configuration`
pub trait Configuration: Debug {
	/// get all sub configurations
	fn sub_configurations(&self) -> Vec<Box<dyn Configuration>>;
}
// endregion:   --- Configuration
