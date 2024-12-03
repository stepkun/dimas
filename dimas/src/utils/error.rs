// Copyright © 2024 Stephan Kunz

//! `dimas` errors

use thiserror::Error;

// region:		--- Error
/// `dimas` error type
#[derive(Error, Debug)]
pub enum Error {
	/// library file not found
	#[error("library not found")]
	NotFound,
	/// register of a library failed
	#[error("register library failed")]
	RegisterFailed,
	/// unloading of a library failed
	#[error("unload of library failed")]
	UnloadFailed,
	/// deregister of a library failed
	#[error("deregister library failed")]
	DeregisterFailed,
}
// region:		--- Error
