// Copyright © 2024 Stephan Kunz

//! Module `message_types` provides the different types of `Message`s used in callbacks.

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::error::Error;
use alloc::{boxed::Box, string::ToString, vec::Vec};
use anyhow::Result;
use bitcode::{decode, encode, Decode, Encode};
use core::ops::Deref;
use zenoh::{query::Query, Wait};
// endregion:	--- modules

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

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<QueryMsg>();
	}
}
