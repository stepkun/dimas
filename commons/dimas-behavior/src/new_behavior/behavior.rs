// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! [`Behavior`] implementation
//!

// region:      --- modules
use alloc::{boxed::Box, string::ToString};
use core::str::FromStr;

use crate::{blackboard::Blackboard, port::PortRemapping};

use super::{BehaviorCreationFn, BehaviorResult, NewBehaviorStatus, NewBehaviorType};
// endregion:   --- modules

// region:      --- BehaviorCreation
/// Methods needed for [`Behavior`] creation
pub trait BehaviorCreation {
	/// Provide the boxed creation function
	fn create() -> Box<BehaviorCreationFn>;
	/// Get the kind of the [`Behavior`] that shall become a Node in a [`BehaviorSubTree`]
	fn kind() -> NewBehaviorType;
}
// endregion:   --- BehaviorCreation

// region:      --- BehaviorTickData
/// Holds the often used Data of a [`Behavior`].
/// Access to members is public for maximum performance.
#[derive(Debug, Default)]
pub struct BehaviorTickData {
	/// Defaults to '0'
	pub child_idx: usize,
	/// Defaults to 'false'
	pub all_skipped: bool,
	/// Current [`BehaviorStatus`]
	pub status: NewBehaviorStatus,
	/// [`Blackboard`] for this [`Behavior`]
	pub blackboard: Blackboard,
	/// In ports including remapping
	pub input_ports: PortRemapping,
	/// Out ports including remapping
	pub output_ports: PortRemapping,
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

	/// Get value of an input port.
	/// # Errors
	pub fn get_input<T>(&self, port: &str) -> BehaviorResult<T>
	where
		T: FromStr + Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		todo!()
	}

	/// Set value of an output port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_output<T>(&self, port: &str, value: T) -> BehaviorResult<()>
	where
		T: Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		todo!()
	}
}
// endregion:   --- BehaviorTickData
