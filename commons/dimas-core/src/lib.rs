// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// see: https://robmosys.eu/wiki/start

#[doc(hidden)]
/// Enums
pub mod enums;
/// Error handling
pub mod error;
/// `Message`, `Request`, `Response`, `Feedback`
pub mod message_types;
/// Operational
mod operational;
#[doc(hidden)]
/// Traits
pub mod traits;
/// Utilities
pub mod utils;

// flatten:
pub use enums::{Signal, TaskSignal};
pub use operational::{OperationState, Operational};
#[doc(hidden)]
pub use traits::{
	Capability, CapabilityDescription, Component, ComponentId, ComponentRegistrar, Configuration,
	Connection, System,
};
pub use traits::{Context, ContextAbstraction};
