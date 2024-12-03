// Copyright © 2023 Stephan Kunz

//! core errors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
#[cfg(doc)]
use crate::operational::OperationState;
use alloc::boxed::Box;
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
}
// region:		--- Error
