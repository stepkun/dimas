// Copyright © 2024 Stephan Kunz

//! `dimas-core/bt` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- BehaviorError
/// `dimas-core` bt error type
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum BehaviorError {
	/// @TODO:
	#[error("evaluating expression [{0}] failed")]
	ExpressionEvaluation(String),

	/// @TODO:
	#[error("could not find port [{0}]: {1}")]
	FindPort(String, String),

	/// The index is out of bounds
	#[error("index [{0}] out of bounds")]
	Index(usize),

	/// @TODO:
	#[error("{0}")]
	NodeStructure(String),

	/// @TODO:
	#[error("could not find entry [{0}] in blackboard")]
	NotInBlackboard(String),

	/// @TODO:
	#[error("could not parse value for port [{0}] into specified type [{1}]")]
	ParsePortValue(String, String),

	/// @TODO:
	#[error("child node of [{0}] returned status [{1}] when not allowed")]
	Status(String, String),

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- BehaviorError
