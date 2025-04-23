// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use dimas_core::ConstString;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-blackboard` error type
#[derive(Error, Debug)]
pub enum Error {
	/// Port already in [`PortList`]
	#[error("name [{0}] is already in port list")]
	AlreadyInPortList(ConstString),
	/// Port already in [`PortRemappings`]
	#[error("name [{0}] is already in remappings")]
	AlreadyInRemappings(ConstString),
	/// Name for a port is not allowed
	#[error("name [{0}] is not allowed for a port")]
	NameNotAllowed(ConstString),
	/// Port not in [`PortList`]
	#[error("name [{0}] is not in list of ports")]
	NotFoundInPortList(ConstString),
	/// Port not in [`PortRemappings`]
	#[error("name [{0}] is not in remappings")]
	NotFoundInRemappings(ConstString),

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(ConstString, ConstString, u32),
}
// region:		--- Error
