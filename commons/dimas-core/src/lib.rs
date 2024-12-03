// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// see: https://robmosys.eu/wiki/start

/// Activity
mod activity;
/// Component, ComponentRegistry
mod component;
/// Enums
pub mod enums;
/// Error handling
pub mod error;
/// `Message`, `Request`, `Response`, `Feedback`
pub mod message_types;
/// Operational
mod operational;
/// Traits
pub mod traits;
/// Utilities
pub mod utils;

// flatten:
pub use activity::{Activity, ActivityType};
pub use component::{Component, ComponentId, ComponentType};
pub use enums::{Signal, TaskSignal};
pub use operational::{OperationState, Operational, OperationalType, Transitions};
pub use traits::{Capability, CapabilityDescription, Configuration, Connection, System};
pub use traits::{Context, ContextAbstraction};
