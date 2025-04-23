// Copyright © 2025 Stephan Kunz

//! [`Behavior`] implementation
//!

// region:      --- modules
use alloc::{
	format,
	string::ToString,
};
use dimas_core::ConstString;
use core::{any::TypeId, str::FromStr};

use crate::{
	blackboard::Blackboard,
	port::{error::Error, get_remapped_key, is_bb_pointer, strip_bb_pointer, NewPortDirection, PortRemappings},
};

use super::{BehaviorResult, BehaviorStatus, error::BehaviorError};
// endregion:   --- modules

// region:		--- BehaviorConfigurationData
/// Holds the Behavior data used during configuration
/// and on other rare occasions.
#[derive(Debug)]
pub struct BehaviorConfigurationData {
	name: ConstString,
}

impl Default for BehaviorConfigurationData {
	fn default() -> Self {
		Self {
			name: "uninitialized".into(),
		}
	}
}

impl BehaviorConfigurationData {
	/// Constructor with name
	#[must_use]
	pub fn new(name: &str) -> Self {
		Self { name: name.into() }
	}

	/// Set name
	pub fn set_name(&mut self, name: &str) {
		self.name = name.into();
	}

	/// Get name
	#[must_use]
	pub const fn name(&self) -> &str {
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
	pub(crate) status: BehaviorStatus,
	/// [`Blackboard`] for this [`Behavior`]
	pub(crate) blackboard: Blackboard,
	/// Ports including remapping
	pub(crate) remappings: PortRemappings,
}
impl BehaviorTickData {
	/// Constructor
	#[must_use]
	pub fn new(blackboard: Blackboard) -> Self {
		Self {
			blackboard,
			..Default::default()
		}
	}

	/// Adds a port to the config based on the direction
	/// # Errors
	/// - if port is already in remappings
	pub fn add_port(&mut self, name: &str, direction: NewPortDirection, value: &str) -> Result<(), Error>{
		self.remappings.add(name, direction, value)
	}

	/// Get value of an input port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	#[allow(clippy::option_if_let_else)]
	pub fn get_input<T>(&self, port_name: &str) -> BehaviorResult<T>
	where
		T: FromStr + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		// extern crate std;
		// std::dbg!("test: {}", &self.blackboard);
		if let Some(remapped_name) = self
			.remappings
			.find(port_name, NewPortDirection::In)
		{
			// entry found
			if remapped_name.is_empty() {
				todo!()
			} else {
				match get_remapped_key(port_name, &remapped_name) {
					// Value is a Blackboard pointer
					Some(key) => self
						.blackboard
						.get_stringy::<T>(&key)
						.map_or_else(|| Err(BehaviorError::NotInBlackboard(key)), |val| Ok(val)),
					// Value is just a normal string
					None => <T as FromStr>::from_str(&remapped_name).map_or_else(
						|_| {
							Err(BehaviorError::ParsePortValue(
								port_name.into(),
								format!("{:?}", TypeId::of::<T>()).into(),
							))
						},
						|val| Ok(val),
					),
				}
			}
		} else {
			// no entry found
			Err(BehaviorError::PortNotDeclared(
				port_name.into(),
				"todo in behavior.rs get_input()".into(),
			))
		}
	}

	/// Set value of an output port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_output<T>(&self, port: &str, value: T) -> BehaviorResult<()>
	where
		T: Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		let port_name = port.to_string();
		if let Some(remapped_name) = self
			.remappings
			.find(port, NewPortDirection::Out)
		{
			// entry found
			let blackboard_key = match &*remapped_name {
				"=" => port_name,
				value => {
					if is_bb_pointer(value) {
						strip_bb_pointer(value).unwrap_or_else(|| todo!()).to_string()
					} else {
						value.to_string()
					}
				}
			};

			self.blackboard.set(blackboard_key, value);

			Ok(())
		} else {
			// entry not found
			Err(BehaviorError::Internal(
				(port_name + " could not set in Blackboard, possibly not defined as output").into(),
			))
		}
	}
}
// endregion:   --- BehaviorTickData
