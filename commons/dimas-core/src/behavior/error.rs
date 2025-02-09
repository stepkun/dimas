// Copyright © 2024 Stephan Kunz

//! `dimas-core` behavior errors

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
pub enum BehaviorError {
	/// Pass through blackboard error
	#[error("{0}")]
	Blackboard(#[from] crate::blackboard::error::Error),

	/// Error in structural composition of a behaviors children
	#[error("{0}")]
	Composition(String),

	/// Pass through float parsing error
	#[error("{0}")]
	FloatParse(#[from] core::num::ParseFloatError),

	/// Evaluation of a scripting expression failed
	#[error("evaluating expression [{0}] failed")]
	ExpressionEvaluation(String),

	/// Port is not in port list
	#[error("could not find port [{0}]")]
	FindPort(String),

	/// Port is not in port list
	#[error("could not find default for port [{0}]")]
	FindPortDefault(String),

	/// The index is out of bounds
	#[error("index [{0}] out of bounds")]
	Index(usize),

	/// Error in internal composition of a behavior
	#[error("{0}")]
	Internal(String),

	/// Variable is not in Blackboard
	#[error("could not find entry [{0}] in blackboard")]
	NotInBlackboard(String),

	/// Port has not been defined in behavior
	#[error("port [{0}] is not declared in behavior [{1}]")]
	PortNotDeclared(String, String),

	/// Type mismatch between port definitin and found value
	#[error("could not parse value for port [{0}] into specified type [{1}]")]
	ParsePortValue(String, String),

	/// Behavior returns a status that is not allowed in this situation
	#[error("child node of [{0}] returned status [{1}] when not allowed")]
	Status(String, String),

	/// Something happened that should not have been possible
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- BehaviorError
