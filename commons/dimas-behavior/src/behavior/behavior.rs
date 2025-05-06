// Copyright © 2025 Stephan Kunz

//! [`Behavior`] implementation
//!

// region:      --- modules
use dimas_core::ConstString;

use super::BehaviorStatus;
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
#[derive(Debug, Default)]
pub struct BehaviorTickData {
	/// Current [`BehaviorStatus`]
	status: BehaviorStatus,
}

impl BehaviorTickData {
	/// Get the current status.
	#[must_use]
	pub const fn status(&self) -> BehaviorStatus {
		self.status
	}

	/// Set the current status.
	pub fn set_status(&mut self, status: BehaviorStatus) {
		self.status = status;
	}
}
// endregion:   --- BehaviorTickData
