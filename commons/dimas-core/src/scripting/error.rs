// Copyright © 2025 Stephan Kunz

//! `scripting` errors

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
	/// @TODO:
	#[error("unknown Operation Code")]
	UnknownOpCode,
}
// region:		--- Error
