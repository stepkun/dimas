// Copyright © 2023 Stephan Kunz

//! core errors
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
#[cfg(doc)]
use super::enums::OperationState;
use alloc::{boxed::Box, string::String};
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` error type.
#[derive(Error, Debug)]
pub enum Error {
	/// decoding failed
	#[error("decoding failed: reason {source}")]
	Decoding {
		/// the original bitcode error
		source: Box<dyn core::error::Error + Send + Sync>,
	},
	/// sending reply failed
	#[error("sending a reply failed: reason {source}")]
	Reply {
		/// the original zenoh error
		source: Box<dyn core::error::Error + Send + Sync>,
	},
	/// empty request
	#[error("query was empty")]
	EmptyQuery,
	/// Not available/implemented
	#[error("no implementation available")]
	NotImplemented,
	/// An unknown [`OperationState`] is given
	#[error("the operation state {state} is unknown")]
	UnknownOperationState {
		/// name of the operation state
		state: String,
	},
	/// library file not found
	#[error("library not found")]
	LibNotFound,
	/// loading of a library file failed
	#[error("load of library failed")]
	LibLoadFailed,
	/// register of a library file failed
	#[error("register library failed")]
	LibRegisterFailed,
	/// unloading of a library file failed
	#[error("unload of file failed")]
	LibUnloadFailed,
	/// deregister of a library file failed
	#[error("register library failed")]
	LibDeregisterFailed,
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
