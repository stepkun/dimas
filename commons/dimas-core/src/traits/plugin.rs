// Copyright © 2024 Stephan Kunz

//! Plugin interface for `DiMAS` components
//!

// see: https://github.com/AndrewGaspar/rust-plugin-example/tree/master

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::string::String;
use core::fmt::Debug;

use super::Operational;
// endregion:	--- modules

// region:		--- types
#[allow(clippy::module_name_repetitions)]
pub type PluginId = String;
// endregion:	--- types

// region:		--- Plugin
/// Contract for a [`Plugin`]
/// [`Plugin`]s must also be [`Operational`]
pub trait Plugin: Debug + Operational + Send + Sync {
	fn id(&self) -> PluginId;
}
// endregion:   --- Plugin
