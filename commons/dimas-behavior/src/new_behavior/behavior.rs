// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! [`Behavior`] implementation
//!

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
};
use core::str::FromStr;

use crate::{blackboard::Blackboard, new_blackboard::NewBlackboard, new_port::NewPortList};

use super::{BehaviorCreationFn, BehaviorResult, NewBehaviorStatus, NewBehaviorType};
// endregion:   --- modules

// region:		--- BehaviorConfigurationData
/// Holds the Behavior data used during configuration
/// and on other rare occasions.
#[derive(Default, Debug)]
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
/// Access to members is public for maximum performance.
#[derive(Debug, Default)]
pub struct BehaviorTickData {
	/// Current [`BehaviorStatus`]
	pub status: NewBehaviorStatus,
	/// [`Blackboard`] for this [`Behavior`]
	pub blackboard: NewBlackboard,
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

	/// Get value of an input port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn get_input<T>(&self, port: impl Into<String>) -> BehaviorResult<T>
	where
		T: FromStr + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		todo!()
	}

	/// Set value of an output port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_output<T>(&self, port: impl Into<String>, value: T) -> BehaviorResult<()>
	where
		T: Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		todo!()
	}
}
// endregion:   --- BehaviorTickData
