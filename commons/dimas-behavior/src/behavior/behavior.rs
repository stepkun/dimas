// Copyright © 2025 Stephan Kunz

//! [`Behavior`] implementation
//!

// region:      --- modules
use alloc::string::{String, ToString};
use core::str::FromStr;
use dimas_core::ConstString;

use crate::blackboard::{BlackboardInterface, BlackboardNodeRef};

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
	pub status: BehaviorStatus,
	/// [`BlackboardNodeRef`] for this [`Behavior`]
	pub(crate) blackboard: BlackboardNodeRef,
}
impl BehaviorTickData {
	/// Constructor
	#[must_use]
	pub fn new(blackboard: BlackboardNodeRef) -> Self {
		Self {
			blackboard,
			..Default::default()
		}
	}

	/// Get value of an input port.
	/// # Errors
	pub fn get_input<T>(&self, port_name: &str) -> BehaviorResult<T>
	where
		T: FromStr + ToString + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		self.blackboard.get::<T>(port_name).map_or_else(
			|_err| {
				self.blackboard
					.get::<String>(port_name)
					.map_or_else(
						|_| Err(BehaviorError::NotInBlackboard(port_name.into())),
						|s| {
							T::from_str(&s).map_or_else(
								|_| Err(BehaviorError::NotInBlackboard(port_name.into())),
								|val| Ok(val),
							)
						},
					)
			},
			|val| Ok(val),
		)
	}

	/// Set value of an output port.
	/// # Errors
	pub fn set_output<T>(&mut self, port_name: &str, value: T) -> BehaviorResult<()>
	where
		T: FromStr + ToString + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		self.blackboard.set(port_name, value)?;

		Ok(())
	}
}
// endregion:   --- BehaviorTickData
