// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! `dimas-blackboard` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;

use crate::behavior::error::BehaviorError;
// endregion:	--- modules

// region:		--- Error
/// `dimas-blackboard` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("Couldn't find port [{0}]")]
	PortError(String),

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- Error
