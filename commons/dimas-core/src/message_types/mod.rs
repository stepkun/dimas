// Copyright © 2024 Stephan Kunz

//! Module `message_types` provides the different types of `Message`s used in callbacks.

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::error::{Error, Result};
use alloc::{boxed::Box, string::ToString, vec::Vec};
use bitcode::{Decode, Encode, decode, encode};
use core::ops::Deref;
use zenoh::{Wait, query::Query};
// endregion:	--- modules

// region:		--- Message
/// Implementation of a [`Message`].
#[derive(Debug)]
pub struct Message(Vec<u8>);

impl Deref for Message {
	type Target = Vec<u8>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Clone for Message {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl Message {
	/// Create a Message from raw data
	#[must_use]
	pub const fn new(value: Vec<u8>) -> Self {
		Self(value)
	}

	/// Encode Message
	#[must_use]
	pub fn encode<T>(message: &T) -> Self
	where
		T: Encode,
	{
		let content = encode(message);
		Self(content)
	}

	/// Decode Message
	///
	/// # Errors
	pub fn decode<T>(self) -> Result<T>
	where
		T: for<'a> Decode<'a>,
	{
		let value: Vec<u8> = self.0;
		decode::<T>(value.as_slice()).map_err(|source| {
			Error::Decoding {
				source: Box::new(source),
			}
			.into()
		})
	}

	/// Get value of [`Message`]
	#[must_use]
	pub const fn value(&self) -> &Vec<u8> {
		&self.0
	}
}
// endregion:	--- Message

// region:    	--- QueryMsg
/// Implementation of a `Query` message handled by a `Queryable`
#[derive(Debug)]
pub struct QueryMsg(pub Query);

impl Clone for QueryMsg {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl Deref for QueryMsg {
	type Target = Query;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl QueryMsg {
	/// Reply to the given [`QueryMsg`]
	///
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn reply<T>(self, value: T) -> Result<()>
	where
		T: Encode,
	{
		let key = self.0.selector().key_expr().to_string();
		let encoded: Vec<u8> = encode(&value);

		self.0
			.reply(&key, encoded)
			.wait()
			.map_err(|source| Error::Reply { source })?;
		Ok(())
	}

	/// Access the queries parameters
	#[must_use]
	pub fn parameters(&self) -> &str {
		self.0.parameters().as_str()
	}

	/// Decode [`QueryMsg`]
	///
	/// # Errors
	pub fn decode<T>(&self) -> Result<T>
	where
		T: for<'a> Decode<'a>,
	{
		if let Some(value) = self.0.payload() {
			let content: Vec<u8> = value.to_bytes().into_owned();
			return decode::<T>(content.as_slice()).map_err(|source| {
				Error::Decoding {
					source: Box::new(source),
				}
				.into()
			});
		}
		Err(Error::EmptyQuery.into())
	}
}
// endregion: 	--- QueryMsg

// region:		--- QueryableMsg
/// Implementation of a `Queryable` message handled by a `Query`
#[derive(Debug)]
pub struct QueryableMsg(pub Vec<u8>);

impl Clone for QueryableMsg {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl Deref for QueryableMsg {
	type Target = Vec<u8>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl QueryableMsg {
	/// Encode [`QueryableMsg`]
	pub fn encode<T>(message: &T) -> Self
	where
		T: Encode,
	{
		let content = encode(message);
		Self(content)
	}

	/// Decode [`QueryableMsg`]
	///
	/// # Errors
	pub fn decode<T>(self) -> Result<T>
	where
		T: for<'a> Decode<'a>,
	{
		let value: Vec<u8> = self.0;
		decode::<T>(value.as_slice()).map_err(|source| {
			Error::Decoding {
				source: Box::new(source),
			}
			.into()
		})
	}
}
// endregion:	--- QueryableMsg

// region:		--- ControlResponse
#[derive(Debug, Encode, Decode)]
/// ?
pub enum ControlResponse {
	/// ?
	Accepted,
	/// ?
	Canceled,
	/// ?
	Declined,
	/// ?
	Occupied,
}
// endregion:	--- ControlResponse

// region:		--- ObservableResponse
#[derive(Debug, Encode, Decode)]
/// ?
pub enum ObservableResponse {
	/// ?
	Canceled(Vec<u8>),
	/// ?
	Feedback(Vec<u8>),
	/// ?
	Finished(Vec<u8>),
}
// endregion:	--- ObservableResponse

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Message>();
		is_normal::<QueryMsg>();
		is_normal::<QueryableMsg>();
		is_normal::<ControlResponse>();
		is_normal::<ObservableResponse>();
	}
}
