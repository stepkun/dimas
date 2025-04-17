// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port module

mod error;
#[allow(clippy::module_inception)]
mod port;

// flatten
pub use port::{
	NewPort, NewPortDefinition, NewPortDirection, get_remapped_key, input_port, is_bb_pointer,
	output_port, port_list, strip_bb_pointer,
};

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
