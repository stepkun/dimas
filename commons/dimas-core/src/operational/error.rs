// Copyright © 2024 Stephan Kunz

//! [`Operational`] & [`OperationState`] errors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` error type.
#[derive(Error, Debug)]
pub enum Error {
	/// manage operation state failed
	#[error("managing operation state failed")]
	ManageState,

	/// the integer representation could not be parsed
	#[error("cannot parse {value} into an OperationState")]
	ParseInt {
		/// integer value of the operation state
		value: i32,
	},
	/// An unknown [`OperationState`] is given
	#[error("the operation state {state} is unknown")]
	UnknownOperationState {
		/// name of the operation state
		state: String,
	},
}
// region:		--- Error
