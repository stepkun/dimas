// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! Activity interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use core::fmt::Debug;

use alloc::{boxed::Box, string::String};

use crate::Operational;
// endregion:	--- modules

// region:		--- types
/// An identifier for an [`Activity`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ActivityId = String;
// endregion:	--- types

// region:		--- Activity
/// Contract for an `Activity`
pub trait Activity: Operational + Debug + Send + Sync {
	/// Get [`Activity`]s id
	fn id(&self) -> ActivityId;
}
// endregion:	--- Activity

// region:		--- ActivityFactory
/// Contract for [`Activity`] factories
pub trait ActivityFactory {
	/// Factory method for [`Activity`] factories
	fn create_activity(id: ActivityId) -> Box<dyn Activity>;
}
// endregion:	---	ActivityFactory
