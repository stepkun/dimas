// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port module

pub mod error;
mod port_definition;
mod port_list;
#[allow(clippy::module_inception)]
mod port_remappings;

use core::any::TypeId;

use dimas_core::BoxConstString;
use error::Error;

// flatten
pub use port_definition::PortDefinition;
pub use port_list::PortList;
pub use port_remappings::PortRemappings;

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
pub fn resolve_special_port(port_name: &str, remapped_port: &str) -> Option<BoxConstString> {
	// is the shortcut '{=}' used?
	if port_name == "{=}" || remapped_port == "{=}" {
		Some(port_name.into())
	} else {
		strip_bb_pointer(remapped_port)
	}
}

/// Remove all 'decoration' from port name
#[must_use]
pub fn strip_bb_pointer(port: &str) -> Option<BoxConstString> {
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
	direction: PortDirection,
	name: &str,
	default: &str,
	description: &str,
) -> Result<PortDefinition, Error> {
	if is_allowed_port_name(name) {
		let type_id = TypeId::of::<T>();
		Ok(PortDefinition {
			_direction: direction,
			_type_id: type_id,
			name: name.into(),
			_default_value: default.into(),
			_description: description.into(),
		})
	} else {
		Err(Error::NameNotAllowed(name.into()))
	}
}

/// Check a name to be allowed for ports
/// # Panics
/// - if something weird happens.
#[must_use]
pub fn is_allowed_port_name(name: &str) -> bool {
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

// region:      --- PortDirection
/// Direction of a `Port`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PortDirection {
	/// Input port
	In,
	/// Output port
	Out,
	/// Bidirecional port
	InOut,
}

impl core::fmt::Display for PortDirection {
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

// region:		---macros
/// macro for creation of an input port definition
#[macro_export]
macro_rules! input_port_macro {
	($tp:ty, $name:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::In, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::In, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::In, $name, $default, $desc)
			.expect("snh")
	};
}

/// macro for creation of an in/out port definition
#[macro_export]
macro_rules! inout_port_macro {
	($tp:ty, $name:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::InOut, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::InOut, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::InOut, $name, $default, $desc)
			.expect("snh")
	};
}

/// macro for creation of an output port definition
#[macro_export]
macro_rules! output_port_macro {
	($tp:ty, $name:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::Out, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::Out, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal $(,)?) => {
		$crate::port::create_port::<$tp>($crate::port::PortDirection::Out, $name, $default, $desc)
			.expect("snh")
	};
}

/// macro for creation of a [`PortList`]
#[macro_export]
macro_rules! port_list {
	($($e:expr),* $(,)?) => {$crate::port::PortList(vec![$($e)*])};
}
// endregion:	--- macros
