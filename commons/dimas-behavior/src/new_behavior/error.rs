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
	/// The index of a behavior is out of bounds
	#[error("index [{0}] out of bounds")]
	IndexOutOfBounds(usize),
	/// Error in internal composition of a behavior
	#[error("{0}")]
	Internal(String),
	/// Variable/Port is not in Blackboard
	#[error("could not find entry [{0}] in blackboard")]
	NotInBlackboard(String),
	/// Type mismatch between port definiton and found value
	#[error("could not parse value for port [{0}] into specified type [{1}]")]
	ParsePortValue(String, String),
	/// Port has not been defined in behavior
	#[error("port [{0}] is not declared in behavior [{1}]")]
	PortNotDeclared(String, String),
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
