// Copyright © 2024 Stephan Kunz

//! `builder` errors

use thiserror::Error;

// region:		--- Error
/// `builder` error type
#[derive(Error, Debug)]
pub enum Error {
	/// No zenoh available/implemented
	#[error("no zenoh session available")]
	NoZenohSession,
}
// region:		--- Error
