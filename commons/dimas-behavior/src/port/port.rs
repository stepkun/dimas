// Copyright © 2024 Stephan Kunz

//! `dimas-behaviortree` port

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
};
use core::{
	any::Any,
	fmt::{Debug, Display, Formatter},
};
use hashbrown::HashMap;

use crate::blackboard::BlackboardString;
// endregion:   --- modules

// region:      --- helper
/// @TODO:
pub fn get_remapped_key(
	port_name: impl AsRef<str>,
	remapped_port: impl AsRef<str>,
) -> Option<String> {
	if port_name.as_ref() == "{=}" || remapped_port.as_ref() == "{=}" {
		Some(port_name.as_ref().to_string())
	} else {
		remapped_port.as_ref().strip_bb_pointer()
	}
}
// endregion:   --- helper

// region:      --- traits
/// Trait for checking a port
#[allow(clippy::module_name_repetitions)]
pub trait PortChecks {
	/// @TODO:
	fn is_allowed_port_name(&self) -> bool;
}

impl<T: AsRef<str>> PortChecks for T {
	fn is_allowed_port_name(&self) -> bool {
		let name = self.as_ref();

		if name.is_empty() {
			false
		} else if name == "_autoremap" {
			true
		} else if !name
			.chars()
			.next()
			.unwrap_or_else(|| todo!())
			.is_ascii_alphabetic()
		{
			false
		} else {
			// If the name isn't name or ID, it's valid
			!(name == "name" || name == "ID")
		}
	}
}

/// Trait for cloning of ports
#[allow(clippy::module_name_repetitions)]
pub trait PortClone {
	/// @TODO:
	fn clone_port(&self) -> Box<dyn PortValue>;
}

impl<T> PortClone for T
where
	T: 'static + Any + Clone + Debug + ToString,
{
	fn clone_port(&self) -> Box<dyn PortValue> {
		Box::new(self.clone())
	}
}

/// Trait to ensure properties of a port
#[allow(clippy::module_name_repetitions)]
pub trait PortValue: Any + PortClone + Debug + ToString {}

impl<T> PortValue for T where T: Any + PortClone + Debug + ToString {}
// endregion:   --- traits

// region:      --- types
/// List of ports
#[allow(clippy::module_name_repetitions)]
pub type PortList = HashMap<String, Port>;

/// Remapping list
#[allow(clippy::module_name_repetitions)]
pub type PortRemapping = HashMap<String, String>;
// endregion:   --- types

// region:      --- PortDirection
/// @TODO:
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDirection {
	/// @TODO:
	Input,
	/// @TODO:
	Output,
	/// @TODO:
	InOut,
}

impl Display for PortDirection {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::Input => "Input",
			Self::Output => "Output",
			Self::InOut => "InOut",
		};

		write!(f, "{text}")
	}
}
// endregion:   --- PortDirection

// region:      --- Port
#[derive(Clone, Debug)]
/// @TODO:
pub struct Port {
	r#type: PortDirection,
	description: String,
	default_value: Option<String>,
	/// When `true`, should parse the port value as an expression when loading
	/// the tree to validate syntax.
	parse_expr: bool,
}

impl Port {
	/// @TODO:
	#[must_use]
	pub const fn new(direction: PortDirection) -> Self {
		Self {
			r#type: direction,
			description: String::new(),
			default_value: None,
			parse_expr: false,
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn default_value(&self) -> Option<&String> {
		match &self.default_value {
			Some(v) => Some(v),
			None => None,
		}
	}

	/// @TODO:
	#[allow(clippy::redundant_closure_for_method_calls)]
	#[must_use]
	pub fn default_value_str(&self) -> Option<String> {
		self.default_value.as_ref().map(|v| v.to_string())
	}

	/// @TODO:
	#[must_use]
	pub const fn direction(&self) -> &PortDirection {
		&self.r#type
	}

	/// @TODO:
	#[must_use]
	pub const fn parse_expr(&self) -> bool {
		self.parse_expr
	}

	/// @TODO:
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_default(&mut self, default: impl ToString) {
		self.default_value = Some(default.to_string());
	}

	/// @TODO:
	pub fn set_description(&mut self, description: String) {
		self.description = description;
	}

	/// @TODO:
	pub const fn set_expr(&mut self, parse_expr: bool) {
		self.parse_expr = parse_expr;
	}
}
// endregion:   --- Port
