// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! [`Activity`] implementation for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use core::fmt::Debug;

use super::Activity;
// endregion:	--- modules

// region:		--- ActivityType
/// Contract for an `Activity`
#[derive(Clone, Debug)]
pub struct ActivityType {}
// endregion:	--- ActivityType

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::boxed::Box;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Box<dyn Activity>>();
	}
}
