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
	/// a Mutex is poisoned.
	#[error("a Mutex poison error happened in {0}")]
	MutexPoison(String),
}
// region:		--- Error

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Error>();
	}
}
