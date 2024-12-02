// Copyright © 2024 Stephan Kunz

//! `ComponentRegistry` interface for `DiMAS` systems
//!

// see: https://github.com/AndrewGaspar/rust-Component-example/tree/master

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{ComponentId, ComponentType};
// endregion:	--- modules

// region:		--- ComponentRegistrar
/// Contract for registering [`Component`]s
pub trait ComponentRegistry: Debug {
	/// to register a [`Component`]
	fn register(&mut self, component: ComponentType);

	/// to remove a registered [`Component`]
	/// # Errors
	fn deregister(&mut self, id: &ComponentId) -> Result<Option<ComponentType>>;
}
// endregion:   --- ComponentRegistrar
