// Copyright © 2024 Stephan Kunz

//! Plugin interface for `DiMAS` components
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use core::fmt::Debug;
// endregion:	--- modules

// region:		--- Plugin
/// contract for a `Plugin`
pub trait Plugin: Debug {

}
// endregion:   --- Plugin