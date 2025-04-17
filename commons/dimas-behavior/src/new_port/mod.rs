// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port module

mod error;
#[allow(clippy::module_inception)]
mod port;

// flatten
pub use port::{NewPort, NewPortDefinition, input_port, output_port, port_list};

// region:      --- modules
use alloc::string::String;
use hashbrown::HashMap;
// endregion:   --- modules

// region:      --- types
/// List of ports
#[allow(clippy::module_name_repetitions)]
pub type NewPortList = HashMap<String, NewPortDefinition>;

/// Remapping list
#[allow(clippy::module_name_repetitions)]
pub type NewPortRemappings = HashMap<String, String>;
// endregion:   --- types
