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

// region:      --- BehaviorData
/// Holds the often used Data of a [`Behavior`].
/// Access is public for maximum performance.
#[derive(Debug, Default)]
pub struct BehaviorTickData {
	/// Defaults to '0'
	pub child_idx: usize,
	/// Defaults to 'false'
	pub all_skipped: bool,
	/// Current [`BehaviorStatus`]
	pub status: NewBehaviorStatus,
	/// [`Blackboard`] for this [`Behavior`]
	blackboard: Blackboard,
	/// In ports including remapping
	input_ports: PortRemapping,
	/// Out ports including remapping
	output_ports: PortRemapping,
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
		// match self.input_ports.get(port) {
		// 	Some(port_remapped) => {
		// 		// Check if default is needed
		// 		if port_remapped.is_empty() {
		// 			self.manifest().map_or_else(
		// 				|_| Err(BehaviorError::Internal("no manifest found".into())),
		// 				|manifest| {
		// 					let port_info = manifest
		// 						.port_list
		// 						.get(port)
		// 						.ok_or_else(|| BehaviorError::FindPort(port.into()))?;

		// 					port_info.default_value().map_or_else(
		// 						|| Err(BehaviorError::FindPortDefault(port.into())),
		// 						|default| {
		// 							default.parse_str().map_or_else(
		// 								|_| {
		// 									Err(BehaviorError::ParsePortValue(
		// 										port.into(),
		// 										"String".into(),
		// 									))
		// 								},
		// 								|value| Ok(value),
		// 							)
		// 						},
		// 					)
		// 				},
		// 			)
		// 		} else {
		// 			match get_remapped_key(port, port_remapped) {
		// 				// Value is a Blackboard pointer
		// 				Some(key) => self
		// 					.blackboard
		// 					.get_stringy::<T>(&key)
		// 					.map_or_else(
		// 						|| Err(BehaviorError::NotInBlackboard(key)),
		// 						|val| Ok(val),
		// 					),
		// 				// Value is just a normal string
		// 				None => <T as FromStr>::from_str(port_remapped).map_or_else(
		// 					|_| {
		// 						Err(BehaviorError::ParsePortValue(
		// 							String::from(port),
		// 							format!("{:?}", TypeId::of::<T>()),
		// 						))
		// 					},
		// 					|val| Ok(val),
		// 				),
		// 			}
		// 		}
		// 	}
		// 	// Port not found in behaviors port list
		// 	None => Err(BehaviorError::PortNotDeclared(
		// 		String::from(port),
		// 		String::from(&self.path),
		// 	)),
		// }
		todo!()
	}

	/// Set value of an output port.
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_output<T>(&self, port: &str, value: T) -> BehaviorResult<()>
	where
		T: Clone + core::fmt::Debug + Send + Sync + 'static,
	{
		// match self.output_ports.get(port) {
		// 	Some(port_value) => {
		// 		let blackboard_key = match port_value.as_str() {
		// 			"=" => port.to_string(),
		// 			value => {
		// 				if value.is_bb_pointer() {
		// 					value
		// 						.strip_bb_pointer()
		// 						.unwrap_or_else(|| todo!())
		// 				} else {
		// 					value.to_string()
		// 				}
		// 			}
		// 		};

		// 		self.blackboard.set(blackboard_key, value);

		// 		Ok(())
		// 	}
		// 	None => Err(BehaviorError::Internal(
		// 		port.to_string() + "could not set in Blackboard, possibly not defined as output",
		// 	)),
		// }
		Ok(())
	}
}
// endregion:   --- BehaviorData
