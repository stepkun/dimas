// Copyright © 2024 Stephan Kunz

//! `dimas-com` errors

// region:		--- modules
#[cfg(doc)]
use crate::zenoh::{Communicator, Observable, Observer, Publisher, Querier, Queryable, Subscriber};
#[cfg(doc)]
use dimas_core::message_types::Message;
use thiserror::Error;
#[cfg(doc)]
use zenoh::query::Query;
// endregion:	--- modules

// region:		--- Error
/// `dimas-com` error type.
#[derive(Error, Debug)]
pub enum Error {
	/// Write access failed
	#[error("write storage for {0} failed")]
	ModifyStruct(String),
	/// Not available/implemented
	#[error("no implementation available")]
	NotImplemented,
	/// No communicator for that id
	#[error("no communicator with id: {0}")]
	NoCommunicator(String),
	/// Read access failed
	#[error("read storage for {0} failed")]
	ReadAccess(String),
	/// Found unknown communication protocol
	#[error("the protocol '{protocol}' is unknown")]
	UnknownProtocol {
		/// protocol name
		protocol: String,
	},
}
// region:		--- Error
