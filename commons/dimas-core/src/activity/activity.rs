// Copyright © 2024 Stephan Kunz

//! Activity interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;

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
pub trait Activity: Operational + Send + Sync {
	/// Get [`Activity`]s id
	fn id(&self) -> ActivityId;

	/// Get [`Activity`]s id
	fn set_id(&mut self, id: ActivityId);
}
// endregion:	--- Activity
