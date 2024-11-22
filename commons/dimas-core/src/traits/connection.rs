// Copyright © 2024 Stephan Kunz

//! Connection
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use core::fmt::Debug;
// endregion:	--- modules

// region:		--- Connection
/// contract for a `Connection`
pub trait Connection: Debug {

}
// endregion:   --- Connection