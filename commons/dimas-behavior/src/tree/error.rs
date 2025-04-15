// Copyright © 2024 Stephan Kunz

//! `dimas-behavior` tree errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` behavior error type
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum Error {
	/// Pass through behavior error
	#[error("{0}")]
	Behavior(#[from] crate::new_behavior::error::NewBehaviorError),

	/// The index of tree node is out of bounds
	#[error("index [{0}] out of bounds")]
	IndexOutOfBounds(usize),

	/// The root of the tree s not properly created
	#[error("tree root [{0}] not found")]
	RootNotFound(String),

	/// Something happened that should not have been possible
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- Error
