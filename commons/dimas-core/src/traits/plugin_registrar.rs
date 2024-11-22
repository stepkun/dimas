// Copyright © 2024 Stephan Kunz

//! PluginRegistrar interface for `DiMAS` systems
//!

// see: https://github.com/AndrewGaspar/rust-plugin-example/tree/master

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::fmt::Debug;

use super::{Plugin, PluginId};
// endregion:	--- modules

// region:		--- PluginRegistrar
/// Contract for registering [`Plugin`]s
pub trait PluginRegistrar: Debug {
	/// to register a [`Plugin`]
	fn register(&mut self, plugin: Box<dyn Plugin>);

	/// to remove a registered [`Plugin`]
	fn deregister(&mut self, id: &PluginId) -> Result<Option<Box<dyn Plugin>>>;

	/// get [`Plugin`]s
	fn plugins(&self) -> impl Iterator<Item = (usize, &Box<dyn Plugin>)>;
}
// endregion:   --- PluginRegistrar
