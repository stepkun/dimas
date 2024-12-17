// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::String};
use core::fmt::Debug;
use uuid::Uuid;

use crate::{Activity, ActivityId, ManageOperationState, Operational};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`Component`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ComponentId = String;
// endregion:	--- types

// region:		--- Component
/// Contract for a [`Component`]
pub trait Component: Operational + ManageOperationState + Debug + Send + Sync {
	/// Get the [`Component`]s unique ID
	fn uuid(&self) -> Uuid;

	/// Get the [`Component`]s id
	fn id(&self) -> ComponentId;

	/// Get the [`Component`]s version
	fn version(&self) -> u32;

	/// Add a sub [`Activity`]
	fn add_activity(&mut self, activity: Box<dyn Activity>);

	/// Remove the sub [`Activity`] with the given `id`
	fn remove_activity(&mut self, id: ActivityId);

	/// Add a sub [`Component`]
	fn add_component(&mut self, component: Box<dyn Component>);

	/// Remove the sub [`Component`] with the given `id`
	fn remove_component(&mut self, id: ComponentId);
}
// endregion:   --- Component
