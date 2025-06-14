// Copyright © 2024 Stephan Kunz

//! `BTFactory` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas` error type
#[derive(Error, Debug)]
pub enum Error {
	/// Value is not a boolean.:
	#[error("Value is not a bool")]
	NoBoolean,

	/// A really unexpected error happened.
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- Error
