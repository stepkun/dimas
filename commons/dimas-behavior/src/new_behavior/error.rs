// Copyright © 2024 Stephan Kunz

//! `dimas-behavior` behavior errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- BehaviorError
/// `dimas-core` behavior error type
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum NewBehaviorError {
	/// The root of the tree is not properly created
	#[error("tree root [{0}] not found")]
	RootNotFound(String),

	/// An illegal [`BehaviorStatus`] is reached
	#[error("child node of [{0}] returned status [{1}] when not allowed")]
	Status(String, String),

	/// Something happened that should not have been possible
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- BehaviorError
