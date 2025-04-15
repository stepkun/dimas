// Copyright © 2025 Stephan Kunz

//! [`Behavior`] library
//!

// #[allow(clippy::module_inception)]
pub mod action;
mod behavior;
pub mod condition;
pub mod control;
pub mod decorator;
pub mod error;
mod simple_behavior;

// flatten
pub use behavior::{BehaviorCreation, BehaviorTickData};
pub use simple_behavior::{BhvrTickFn, SimpleBehavior};

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use error::NewBehaviorError;

use crate::{port::PortList, tree::BehaviorTreeComponent};
// endregion:   --- modules

// region:		--- types
/// Result type definition for [`Behavior`]s
pub type BehaviorResult<Output = NewBehaviorStatus> = Result<Output, NewBehaviorError>;

/// Type alias for a [`Behavior`] creation function
pub type BehaviorCreationFn = dyn Fn() -> Box<dyn BehaviorMethods> + Send + Sync;
// endregion:	--- types

// region:		--- BehaviorMethods
/// Defines the methods common to all [`Behavior`]s.
/// These methods are available when traversing a [`BehaviorTree`].
pub trait BehaviorMethods: core::fmt::Debug + Send + Sync {
	/// Provide the list of available [`Port`]s.
	/// Default implementation returns an empty list.
	fn ports(&self) -> PortList {
		PortList::default()
	}

	/// Method called to start ticking a [`Behavior`].
	/// Defaults to calling `self.tick(...)`
	/// # Errors
	fn start(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut Vec<BehaviorTreeComponent>,
	) -> BehaviorResult {
		self.tick(tick_data, children)
	}

	/// Method called to tick a [`Behavior`].
	/// # Errors
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut Vec<BehaviorTreeComponent>,
	) -> BehaviorResult;

	/// Method called to stop/cancel/halt a [`Behavior`].
	/// Default implementation just returns [`BehaviorStatus::Idle`]
	/// # Errors
	#[allow(unused_variables)]
	fn halt(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut Vec<BehaviorTreeComponent>,
	) -> BehaviorResult {
		Ok(NewBehaviorStatus::Idle)
	}
}
// endregion:	--- BehaviorMethods

// region:      --- BehaviorStatus
/// Behavior status
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum NewBehaviorStatus {
	/// Behavior execution failed.
	Failure,
	/// Behavior is not executing.
	#[default]
	Idle,
	/// Behavior is still executing.
	Running,
	/// Behavior has been skipped.
	Skipped,
	/// Behavior finished with success.
	Success,
}

impl NewBehaviorStatus {
	/// Create colourized output for modern terminals
	#[must_use]
	pub fn into_string_color(&self) -> String {
		let color_start = match self {
			Self::Failure => "\x1b[31m",
			Self::Idle => "\x1b[36m",
			Self::Running => "\x1b[33m",
			Self::Skipped => "\x1b[34m",
			Self::Success => "\x1b[32m",
		};

		color_start.to_string() + &self.to_string() + "\x1b[0m"
	}

	/// Check if status is signaling that the behavior is active
	#[must_use]
	pub const fn is_active(&self) -> bool {
		matches!(self, Self::Idle | Self::Skipped)
	}

	/// Check if status is signaling that the behavior is completed
	#[must_use]
	pub const fn is_completed(&self) -> bool {
		matches!(self, Self::Success | Self::Failure)
	}
}

impl core::fmt::Display for NewBehaviorStatus {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::Failure => "Failure",
			Self::Idle => "Idle",
			Self::Running => "Running",
			Self::Skipped => "Skipped",
			Self::Success => "Success",
		};

		write!(f, "{text}")
	}
}
// endregion:   --- BehaviorStatus

// region:		--- BehaviorType
/// All types of behaviors usable in a behavior tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NewBehaviorType {
	/// Action
	Action,
	/// Condition
	Condition,
	/// Control
	Control,
	/// Decorator
	Decorator,
	/// Subtree
	SubTree,
}

impl core::fmt::Display for NewBehaviorType {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::Action => "Action",
			Self::Condition => "Condition",
			Self::Control => "Control",
			Self::Decorator => "Decorator",
			Self::SubTree => "SubTree",
		};

		write!(f, "{text}")
	}
}
// endregion:	--- BehaviorType
