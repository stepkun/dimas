// Copyright © 2024 Stephan Kunz

//! core traits
//!

#[doc(hidden)]
mod capability;
#[doc(hidden)]
mod capability_description;
#[doc(hidden)]
mod component;
#[doc(hidden)]
mod component_registrar;
#[doc(hidden)]
mod configuration;
#[doc(hidden)]
mod connection;
mod context;
mod operational;
#[doc(hidden)]
mod system;

// flatten
#[doc(hidden)]
pub use capability::Capability;
#[doc(hidden)]
pub use capability_description::CapabilityDescription;
#[doc(hidden)]
pub use component::{Component, ComponentId};
#[doc(hidden)]
pub use component_registrar::ComponentRegistrar;
#[doc(hidden)]
pub use configuration::Configuration;
#[doc(hidden)]
pub use connection::Connection;
pub use context::{Context, ContextAbstraction};
pub use operational::Operational;
#[doc(hidden)]
pub use system::System;
