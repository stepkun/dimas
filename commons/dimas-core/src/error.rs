// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! `dimas-core` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` error type
#[derive(Error, Debug)]
pub enum Error {
	/// Should not happen
	#[error("this should not have happened in file {0} at line {1}")]
	Unexpected(String, u32),
}
// region:		--- Error
