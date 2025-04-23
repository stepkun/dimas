// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Blackboard errors

#[doc(hidden)]
extern crate alloc;

use dimas_core::ConstString;
// region		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-blackboard` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("Couldn't find port [{0}]")]
	PortError(ConstString),

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(ConstString, ConstString, u32),
}
// region:		--- Error
