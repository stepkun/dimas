// Copyright © 2024 Stephan Kunz

//! Module `message_types` provides the different types of `Message`s used in callbacks.

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::error::Error;
use alloc::{boxed::Box, vec::Vec};
use anyhow::Result;
use bitcode::{decode, encode, Decode, Encode};
use core::ops::Deref;
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

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Message>();
	}
}
