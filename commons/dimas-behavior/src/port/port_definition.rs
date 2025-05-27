// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` `PortDefintion` implementation

#[doc(hidden)]
extern crate alloc;

use core::any::TypeId;

// region:      --- modules
use dimas_core::ConstString;

use super::{PortDirection, error::Error, is_allowed_port_name};
// endregion:   --- modules

// region:      --- PortDefinition
/// A static [`PortDefinition`], which is used for configuration.
/// Access to members is public within crate to maximize performance
#[derive(Clone, Debug)]
pub struct PortDefinition {
	/// Directiopn of the port.
	_direction: PortDirection,
	/// Type of the port.
	_type_id: TypeId,
	/// Name of the port.
	name: ConstString,
	/// Default value for the port.
	_default_value: ConstString,
	/// Description of the port.
	_description: ConstString,
}

impl PortDefinition {
	/// Constructor
	/// # Errors
	/// - if the name violates the conventions.
	pub fn new(
		direction: PortDirection,
		type_id: TypeId,
		name: &str,
		default_value: &str,
		description: &str,
	) -> Result<Self, Error> {
		if is_allowed_port_name(name) {
			Ok(Self {
				_direction: direction,
				_type_id: type_id,
				name: name.into(),
				_default_value: default_value.into(),
				_description: description.into(),
			})
		} else {
			Err(Error::NameNotAllowed(name.into()))
		}
	}

	/// Get the [`PortDefinition`]s name.
	#[must_use]
	pub fn name(&self) -> ConstString {
		self.name.clone()
	}
}
// endregion:   --- PortDefinition
