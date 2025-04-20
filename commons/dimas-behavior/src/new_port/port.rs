// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `dimas-behavior` Port implementation

#[doc(hidden)]
extern crate alloc;

use core::{any::TypeId, ops::{Deref, DerefMut}};

// region:      --- modules
use alloc::{string::{String, ToString}, vec::Vec};

use super::error::Error;
// endregion:   --- modules

// region:      --- types
const FORBIDDEN_NAMES: &[&str] = &[
	"name",
	"ID",
	"_autoremap",
	"_failureIf",
	"_successIf",
	"_skipIf",
	"_while",
	"_onHalted",
	"_onFailure",
	"_onSuccess",
	"_post",
];
// endregion:   --- types

// region:      --- helper
/// Function handles the special remapping cases
#[must_use]
pub fn get_remapped_key(port_name: &str, remapped_port: &str) -> Option<String> {
	// is the shortcut '{=}' used?
	if port_name == "{=}" || remapped_port == "{=}" {
		Some(port_name.to_string())
	} else {
		strip_bb_pointer(remapped_port)
	}
}

/// Remove all 'decoration' from port name
#[must_use]
pub fn strip_bb_pointer(port: &str) -> Option<String> {
	// Is bb pointer
	if port.starts_with('{') && port.ends_with('}') {
		Some(
			port.strip_prefix('{')
				.unwrap_or_else(|| todo!())
				.strip_suffix('}')
				.unwrap_or_else(|| todo!())
				.to_string(),
		)
	} else {
		None
	}
}

/// Check if it is a port
#[must_use]
pub fn is_bb_pointer(port: &str) -> bool {
	port.starts_with('{') && port.ends_with('}')
}

/// Create a [`PortDefinition`]
/// # Errors
/// - if the name violates the conventions.
pub fn create_port<T: 'static>(
	direction: NewPortDirection,
	name: impl Into<String>,
	default: impl Into<String>,
	description: impl Into<String>,
) -> Result<NewPortDefinition, Error> {
	let name = name.into();
	if is_allowed_name(&name) {
		let type_id = TypeId::of::<T>();
		Ok(NewPortDefinition {
			direction,
			type_id,
			name,
			default_value: default.into(),
			description: description.into(),
		})
	} else {
		Err(Error::NameNotAllowed(name))
	}
}

fn is_allowed_name(name: &str) -> bool {
	if name.is_empty() {
		return false;
	}
	let first = name.chars().next().expect("snh");
	if !first.is_alphabetic() {
		return false;
	}

	if FORBIDDEN_NAMES.contains(&name) {
		return false;
	}
	true
}
// endregion:   --- helper

// region:      --- PortList
/// List of ports
/// The `PortList` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct NewPortList(pub Vec<NewPortDefinition>);

impl Deref for NewPortList {
	type Target = Vec<NewPortDefinition>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for NewPortList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl NewPortList {
	/// Add an entry to the [`PortList`]
	/// # Errors
	/// - if entry already exists
	pub fn add(
		&mut self,
		port_definition: NewPortDefinition,
	) -> Result<(), Error> {
		for entry in &mut *self.0 {
			if entry.name == port_definition.name {
				return Err(Error::AlreadyInPortList(entry.name.clone()));
			}
		}
		self.0.push(port_definition);
		Ok(())
	}

	/// Create a list of the [`Port`] names in the list
	#[must_use]
	pub fn entries(&self) -> String {
		let comma = false;
		let mut result = String::new();
		for entry in &self.0 {
			if comma {
				result += ", ";
			}
			result += &entry.name;
		}
		result
	}

	/// Lookup a [`PortDefinition`]
	/// # Errors
	/// - if no [`PortDefinition`] is found
	pub fn find(&self, name: &str) -> Result<NewPortDefinition, Error> {
		for entry in &self.0 {
			if entry.name == name {
				return Ok(entry.clone());
			}
		}
		Err(Error::NotFoundInPortList(name.into()))
	}
}
// endregion:	--- PortList

// region:		--- PortRemappings
/// Remapping list
/// `PortRemappings` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct NewPortRemappings(Vec<(String, (NewPortDirection, String))>);

impl Deref for NewPortRemappings {
	type Target = Vec<(String, (NewPortDirection, String))>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for NewPortRemappings {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl NewPortRemappings {
	/// Add an entry to the [`PortRemappings`]
	/// # Errors
	/// - if entry already exists
	pub fn add(
		&mut self,
		name: &str,
		direction: NewPortDirection,
		remapped_name: &str,
	) -> Result<(), Error> {
		for (original, _) in &mut *self.0 {
			if original == name {
				return Err(Error::AlreadyInRemappings(name.into()));
			}
		}
		self.push((name.into(), (direction, remapped_name.into())));
		Ok(())
	}

	/// Lookup the remaped name
	#[must_use]
	pub fn find(&self, name: &str, direction: NewPortDirection) -> Option<String> {
		for (original, remapped) in &self.0 {
			if original == name && ((direction == remapped.0) || (remapped.0 == NewPortDirection::InOut)) {
				return Some((remapped.1).clone());
			}
		}
		None
	}
}
// endregion:   --- PortRemappings

// region:      --- PortDirection
/// A [`Port`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NewPortDirection {
	/// Input port
	In,
	/// Output port
	Out,
	/// Bidirecional port
	InOut,
}

impl core::fmt::Display for NewPortDirection {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::In => "Input",
			Self::Out => "Output",
			Self::InOut => "InOut",
		};

		write!(f, "{text}")
	}
}
// endregion:   --- PortDirection

// region:      --- PortDefinition
/// A static [`PortDefinition`], which is used for configuration.
/// Access to members is public within crate to maximize performance
#[derive(Clone, Debug)]
pub struct NewPortDefinition {
	pub(crate) direction: NewPortDirection,
	pub(crate) type_id: TypeId,
	pub(crate) name: String,
	pub(crate) default_value: String,
	pub(crate) description: String,
}

impl NewPortDefinition {
	/// Constructor
	/// # Errors
	/// - if the name violates the conventions.
	pub fn new(
		direction: NewPortDirection,
		type_id: TypeId,
		name: impl Into<String>,
		default_value: impl Into<String>,
		description: impl Into<String>,
	) -> Result<Self, Error> {
		let name = name.into();
		if is_allowed_name(&name) {
			Ok(Self {
				direction,
				type_id,
				name,
				default_value: default_value.into(),
				description: description.into(),
			})
		} else {
			Err(Error::NameNotAllowed(name))
		}
	}
}
// endregion:   --- PortDefinition
