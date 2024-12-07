// Copyright © 2024 Stephan Kunz

//! Contract for every `DiMAS` agent
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use tracing::{event, instrument, Level};

use crate::{
	activity, error::Error, Activity, ActivityId, Component, ComponentId, Configuration,
	Connection, OperationState, Operational,
};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`System`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type SystemId = String;
// endregion:	--- types

// region:		--- System
/// Contract for a `System`
pub trait System: Send + Sync {
	/// Get the [`System`]s unique ID
	fn id(&self) -> SystemId;

	/// Set the [`System`]s unique ID
	fn set_id(&mut self, id: SystemId);
}
// endregion:   --- System
