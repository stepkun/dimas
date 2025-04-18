// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port module

mod error;
#[allow(clippy::module_inception)]
mod port;

use error::Error;
// flatten
pub use port::{
	NewPortDefinition, NewPortDirection, get_remapped_key, input_port, is_bb_pointer, output_port,
	strip_bb_pointer,
};

// region:      --- modules
use alloc::{string::String, vec::Vec};
// endregion:   --- modules

// region:      --- types
/// List of ports
/// The `PortList` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[allow(clippy::module_name_repetitions)]
pub type NewPortList = Vec<NewPortDefinition>;

/// Add an entry to the [`PortRemapping`]s
/// # Errors
/// - if enry already exists
pub fn add_to_port_list(
	list: &mut NewPortList,
	port_definition: NewPortDefinition,
) -> Result<(), Error> {
	for entry in &mut *list {
		if entry.name == port_definition.name {
			return Err(Error::AlreadyInPortList(entry.name.clone()));
		}
	}
	list.push(port_definition);
	Ok(())
}

/// Lookup a [`PortDefinition`]
/// # Errors
/// - if no [`PortDefinition`] found
pub fn find_in_port_list(list: &NewPortList, name: &str) -> Result<NewPortDefinition, Error> {
	for entry in list {
		if entry.name == name {
			return Ok(entry.clone());
		}
	}
	Err(Error::NotFoundInPortList(name.into()))
}

/// Create a list of the [`Port`] names in the list
#[must_use]
pub fn port_list_entries(list: &NewPortList) -> String {
	let comma = false;
	let mut result = String::new();
	for entry in list {
		if comma {
			result += ", ";
		}
		result += &entry.name;
	}
	result
}

/// Remapping list
/// `PortRemappings` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[allow(clippy::module_name_repetitions)]
pub type NewPortRemappings = Vec<(String, String)>;

/// Lookup the remaped name
#[must_use]
pub fn find_in_remapping_list(list: &NewPortRemappings, name: &str) -> Option<String> {
	for (original, remapped) in list {
		if original == name {
			return Some(remapped.clone());
		}
	}
	None
}

/// Add an entry to the [`PortRemapping`]s
/// # Errors
/// - if enry already exists
pub fn add_to_remapping_list(
	list: &mut NewPortRemappings,
	name: &str,
	remapped_name: &str,
) -> Result<(), Error> {
	for (original, _) in &mut *list {
		if original == name {
			return Err(Error::AlreadyInRemappings(name.into()));
		}
	}
	list.push((name.into(), remapped_name.into()));
	Ok(())
}
// endregion:   --- types
