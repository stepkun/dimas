// Copyright © 2024 Stephan Kunz

//! core traits
//!

mod capability;
mod capability_description;
mod configuration;
mod connection;
mod context;

// flatten
pub use capability::Capability;
pub use capability_description::CapabilityDescription;
pub use configuration::Configuration;
pub use connection::Connection;
pub use context::{Context, ContextAbstraction};
