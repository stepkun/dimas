// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// see: https://robmosys.eu/wiki/start

/// States for usage in builders
#[cfg(feature = "std")]
pub mod builder_states;
#[doc(hidden)]
/// Enums
pub mod enums;
/// Error handling
pub mod error;
/// `Message`, `Request`, `Response`, `Feedback`
pub mod message_types;
#[doc(hidden)]
/// Traits
pub mod traits;
/// Utilities
pub mod utils;

// flatten:
pub use enums::{OperationState, Signal, TaskSignal};
#[doc(hidden)]
pub use traits::{
	Capability, CapabilityDescription, Component, Configuration, Connection, Plugin, PluginId,
	PluginRegistrar, System,
};
pub use traits::{Context, ContextAbstraction, Operational};
