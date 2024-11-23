// Copyright © 2024 Stephan Kunz

//! ComponentRegistrar interface for `DiMAS` systems
//!

// see: https://github.com/AndrewGaspar/rust-Component-example/tree/master

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::fmt::Debug;

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentRegistrar
/// Contract for registering [`Component`]s
pub trait ComponentRegistrar: Debug {
	/// to register a [`Component`]
	fn register(&mut self, component: Box<dyn Component>);

	/// to remove a registered [`Component`]
	fn deregister(&mut self, id: &ComponentId) -> Result<Option<Box<dyn Component>>>;
}
// endregion:   --- ComponentRegistrar
