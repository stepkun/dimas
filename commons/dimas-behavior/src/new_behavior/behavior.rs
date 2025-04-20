// Copyright © 2025 Stephan Kunz

//! [`Behavior`] implementation
//!

// region:      --- modules
use alloc::{
	format,
	string::{String, ToString},
};
use core::{any::TypeId, str::FromStr};

use crate::{
	new_blackboard::NewBlackboard,
	new_port::{
		NewPortDirection, NewPortRemappings, get_remapped_key,
		is_bb_pointer, strip_bb_pointer,
	},
};

use super::{BehaviorResult, NewBehaviorStatus, error::NewBehaviorError};
// endregion:   --- modules

// region:		--- BehaviorConfigurationData
/// Holds the Behavior data used during configuration
/// and on other rare occasions.
#[derive(Debug, Default)]
pub struct BehaviorConfigurationData {
	name: String,
}

impl BehaviorConfigurationData {
	/// Constructor with name
	pub fn new(name: impl Into<String>) -> Self {
		Self { name: name.into() }
	}

	/// Set name
	pub fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	/// Get name
	#[must_use]
	pub const fn name(&self) -> &String {
		&self.name
	}
}
// endregion:	--- BehaviorConfigurationData

// region:      --- BehaviorTickData
/// Holds the often used Data of a [`Behavior`].
/// Access to members is public within crate for maximum performance.
#[derive(Debug, Default)]
pub struct BehaviorTickData {
	/// Current [`BehaviorStatus`]
	pub(crate) status: NewBehaviorStatus,
	/// [`Blackboard`] for this [`Behavior`]
	pub(crate) blackboard: NewBlackboard,
	/// Ports including remapping
	pub(crate) remappings: NewPortRemappings,
}
impl BehaviorTickData {
	/// Constructor
	#[must_use]
	pub fn new(blackboard: NewBlackboard) -> Self {
		Self {
			blackboard,
			..Default::default()
		}
	}

	/// Adds a port to the config based on the direction
	pub fn add_port(&mut self, name: String, direction: NewPortDirection, value: String) {
		self.remappings.push((name, (direction, value)));
	}

	/// Get value of an input port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	#[allow(clippy::option_if_let_else)]
	pub fn get_input<T>(&self, port: impl Into<String>) -> BehaviorResult<T>
	where
		T: FromStr + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		// extern crate std;
		// std::dbg!("test: {}", &self.blackboard);
		let port_name = port.into();
		if let Some(remapped_name) = self.remappings.find(&port_name, NewPortDirection::In) {
			// entry found
			if remapped_name.is_empty() {
				todo!()
			} else {
				match get_remapped_key(&port_name, &remapped_name) {
					// Value is a Blackboard pointer
					Some(key) => self
						.blackboard
						.get_stringy::<T>(&key)
						.map_or_else(
							|| Err(NewBehaviorError::NotInBlackboard(key)),
							|val| Ok(val),
						),
					// Value is just a normal string
					None => <T as FromStr>::from_str(&remapped_name).map_or_else(
						|_| {
							Err(NewBehaviorError::ParsePortValue(
								port_name,
								format!("{:?}", TypeId::of::<T>()),
							))
						},
						|val| Ok(val),
					),
				}
			}
		} else {
			// no entry found
			Err(NewBehaviorError::PortNotDeclared(
				port_name,
				String::from("todo in behavior.rs get_input()"),
			))
		}
	}

	/// Set value of an output port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_output<T>(&self, port: impl Into<String>, value: T) -> BehaviorResult<()>
	where
		T: Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		let port_name = port.into();
		if let Some(remapped_name) = self.remappings.find(&port_name, NewPortDirection::Out) {
			// entry found
			let blackboard_key = match remapped_name.as_str() {
				"=" => port_name,
				value => {
					if is_bb_pointer(value) {
						strip_bb_pointer(value).unwrap_or_else(|| todo!())
					} else {
						value.to_string()
					}
				}
			};

			self.blackboard.set(blackboard_key, value);

			Ok(())
		} else {
			// entry not found
			Err(NewBehaviorError::Internal(
				port_name + " could not set in Blackboard, possibly not defined as output",
			))
		}
	}
}
// endregion:   --- BehaviorTickData
