// Copyright © 2023 Stephan Kunz

//! [`Component`] & [`ComponentRegistry`] errors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` error type.
#[derive(Error, Debug)]
pub enum Error {}
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
