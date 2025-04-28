// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port implementation

#[doc(hidden)]
extern crate alloc;

use core::{
	any::TypeId,
	ops::{Deref, DerefMut},
};

// region:      --- modules
use alloc::{string::String, vec::Vec};
use dimas_core::ConstString;

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

/// An immutable remapping entry
type RemappingEntry = (ConstString, (NewPortDirection, ConstString));
// endregion:   --- types

// region:      --- helper
/// Function handles the special remapping cases
#[must_use]
pub fn get_remapped_key(port_name: &str, remapped_port: &str) -> Option<ConstString> {
	// is the shortcut '{=}' used?
	if port_name == "{=}" || remapped_port == "{=}" {
		Some(port_name.into())
	} else {
		strip_bb_pointer(remapped_port)
	}
}

/// Remove all 'decoration' from port name
#[must_use]
pub fn strip_bb_pointer(port: &str) -> Option<ConstString> {
	// Is bb pointer
	if port.starts_with('{') && port.ends_with('}') {
		Some(
			port.strip_prefix('{')
				.unwrap_or_else(|| todo!())
				.strip_suffix('}')
				.unwrap_or_else(|| todo!())
				.into(),
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
	name: &str,
	default: &str,
	description: &str,
) -> Result<NewPortDefinition, Error> {
	if is_allowed_name(name) {
		let type_id = TypeId::of::<T>();
		Ok(NewPortDefinition {
			direction,
			_type_id: type_id,
			name: name.into(),
			_default_value: default.into(),
			_description: description.into(),
		})
	} else {
		Err(Error::NameNotAllowed(name.into()))
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
#[derive(Debug, Default)]
pub struct PortList(pub Vec<NewPortDefinition>);

impl Deref for PortList {
	type Target = Vec<NewPortDefinition>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PortList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PortList {
	/// Add an entry to the [`PortList`]
	/// # Errors
	/// - if entry already exists
	pub fn add(&mut self, port_definition: NewPortDefinition) -> Result<(), Error> {
		for entry in &self.0 {
			if entry.name == port_definition.name {
				return Err(Error::AlreadyInPortList(entry.name.as_ref().into()));
			}
		}
		self.0.push(port_definition);
		Ok(())
	}

	/// Create a list of the [`Port`] names in the list
	#[must_use]
	pub fn entries(&self) -> ConstString {
		let comma = false;
		let mut result = String::new();
		for entry in &self.0 {
			if comma {
				result += ", ";
			}
			result += &entry.name;
		}
		result.into()
	}

	/// Lookup a [`PortDefinition`].
	#[must_use]
	pub fn find(&self, name: &str) -> Option<NewPortDefinition> {
		for entry in &self.0 {
			if &*entry.name == name {
				return Some(entry.clone());
			}
		}
		None
	}
}
// endregion:	--- PortList

// region:		--- PortRemappings
/// Remapping list
/// `PortRemappings` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[derive(Debug, Default)]
pub struct PortRemappings(Vec<RemappingEntry>);

impl PortRemappings {
	/// Add an entry to the [`PortRemappings`]
	/// # Errors
	/// - if entry already exists
	pub fn add(
		&mut self,
		name: &str,
		direction: NewPortDirection,
		remapped_name: &str,
	) -> Result<(), Error> {
		for (original, _) in &self.0 {
			if original.as_ref() == name {
				return Err(Error::AlreadyInRemappings(name.into()));
			}
		}
		self.0
			.push((name.into(), (direction, remapped_name.into())));
		Ok(())
	}

	/// Lookup the remapped name.
	#[must_use]
	pub fn find(&self, name: &str, direction: NewPortDirection) -> Option<ConstString> {
		for (original, remapped) in &self.0 {
			if original.as_ref() == name
				&& ((direction == remapped.0) || (remapped.0 == NewPortDirection::InOut))
			{
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
	pub(crate) _type_id: TypeId,
	pub(crate) name: ConstString,
	pub(crate) _default_value: ConstString,
	pub(crate) _description: ConstString,
}

impl NewPortDefinition {
	/// Constructor
	/// # Errors
	/// - if the name violates the conventions.
	pub fn new(
		direction: NewPortDirection,
		type_id: TypeId,
		name: &str,
		default_value: &str,
		description: &str,
	) -> Result<Self, Error> {
		if is_allowed_name(name) {
			Ok(Self {
				direction,
				_type_id: type_id,
				name: name.into(),
				_default_value: default_value.into(),
				_description: description.into(),
			})
		} else {
			Err(Error::NameNotAllowed(name.into()))
		}
	}
}
// endregion:   --- PortDefinition
