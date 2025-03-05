// Copyright © 2025 Stephan Kunz

//! `scripting` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `scripting` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("unexpected token")]
	UnexpectedToken,
}
// region:		--- Error
