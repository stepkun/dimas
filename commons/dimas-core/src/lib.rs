// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// see: https://robmosys.eu/wiki/start

/// Activity
mod activity;
/// Agent
mod agent;
/// Communication pattern
mod com_pattern;
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
/// System
mod system;
/// Traits
pub mod traits;
/// Utilities
pub mod utils;

// flatten:
pub use activity::{Activity, ActivityId, ActivityType};
pub use agent::Agent;
pub use component::{Component, ComponentData, ComponentId, ComponentType};
pub use enums::{Signal, TaskSignal};
pub use operational::{
	ManageOperationState, OperationState, Operational, OperationalData, OperationalType,
	Transitions,
};
pub use system::{System, SystemId, SystemType};
pub use traits::{Capability, CapabilityDescription, Configuration, Connection};
pub use traits::{Context, ContextAbstraction};
