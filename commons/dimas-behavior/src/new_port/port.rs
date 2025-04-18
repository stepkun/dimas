// Copyright © 2025 Stephan Kunz
#![allow(clippy::needless_pass_by_value)]
#![allow(unused)]

//! `dimas-behavior` Port implementation

#[doc(hidden)]
extern crate alloc;

use core::any::TypeId;

// region:      --- modules
use alloc::{
	string::{String, ToString},
	vec,
};

use super::{NewPortList, error::Error};
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

/// Create a [`PortLists`]
/// # Errors
/// - if a name of a port violates the conventions.
pub fn port_list(port: NewPortDefinition) -> Result<NewPortList, Error> {
	let mut port_list = vec![port];
	Ok(port_list)
}

/// Create a [`PortDefinition`]
/// # Errors
/// - if the name violates the conventions.
fn create_port<T: 'static>(
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

/// Create an input [`PortDefinition`]
/// # Errors
/// - if the name violates the conventions.
pub fn input_port<T: 'static>(
	name: impl Into<String>,
	default: impl Into<String>,
	description: impl Into<String>,
) -> Result<NewPortDefinition, Error> {
	create_port::<T>(NewPortDirection::In, name, default, description)
}

/// Create an output [`PortDefinition`]
/// # Errors
/// - if the name violates the conventions.
pub fn output_port<T: 'static>(
	name: impl Into<String>,
	default: impl Into<String>,
	description: impl Into<String>,
) -> Result<NewPortDefinition, Error> {
	create_port::<T>(NewPortDirection::Out, name, default, description)
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

// region:      --- Port
/// A [`Port`]
#[derive(Clone, Debug)]
pub struct NewPort {
	direction: NewPortDirection,
	name: String,
	description: String,
}
impl NewPort {
	/// Construct a [`Port`]
	/// # Errors
	/// - if the name violates the conventions.
	pub fn new(
		direction: NewPortDirection,
		name: impl Into<String>,
		description: impl Into<String>,
	) -> Result<Self, Error> {
		let name = name.into();
		if is_allowed_name(&name) {
			Ok(Self {
				direction,
				name,
				description: description.into(),
			})
		} else {
			Err(Error::NameNotAllowed(name))
		}
	}
}
// endregion:   --- Port

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
