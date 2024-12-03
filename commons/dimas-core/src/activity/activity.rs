// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! Activity interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use core::fmt::Debug;

use crate::Operational;
// endregion:	--- modules

// region:		--- Activity
/// Contract for an `Activity`
pub trait Activity: Debug + Operational + Send + Sync {
	/// Get [`Activity`]s id
	fn id(&self) -> String;

	/// Get [`Activity`]s id
	fn set_id(&mut self, id: String);
}
// endregion:	--- Activity

#[cfg(test)]
mod tests {
	use alloc::boxed::Box;

	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Box<dyn Activity>>();
	}
}
