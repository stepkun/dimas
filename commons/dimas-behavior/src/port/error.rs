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
	#[error("name [{0}] already in port list")]
	AlreadyInPortList(ConstString),
	/// Port already in [`PortRemappings`]
	#[error("name [{0}] already in remapping list")]
	AlreadyInRemappings(ConstString),
	/// Name for a port is not allowed
	#[error("name [{0}] not allowed for a port")]
	NameNotAllowed(ConstString),
}
// region:		--- Error
