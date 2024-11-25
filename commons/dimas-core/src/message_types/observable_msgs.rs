// Copyright © 2024 Stephan Kunz

//! Module `message_types` provides the different types of `Message`s used in callbacks.

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::vec::Vec;
use bitcode::{Decode, Encode};
// endregion:	--- modules

// region:		--- ObservableControlResponse
#[derive(Debug, Encode, Decode)]
/// ?
pub enum ObservableControlResponse {
	/// ?
	Accepted,
	/// ?
	Canceled,
	/// ?
	Declined,
	/// ?
	Occupied,
}
// endregion:	--- ObservableControlResponse

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
		is_normal::<ObservableControlResponse>();
		is_normal::<ObservableResponse>();
	}
}
