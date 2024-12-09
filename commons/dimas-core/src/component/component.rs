// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{Activity, ActivityId, ManageOperationState};
// endregion:	--- modules

// region:		--- types
/// An identifier for a [`Component`].
/// May be replaced with a more complex struct in future.
#[allow(clippy::module_name_repetitions)]
pub type ComponentId = String;
// endregion:	--- types

// region:		--- Component
/// Contract for a [`Component`]
pub trait Component: ManageOperationState + Send + Sync {
	/// Get the [`Component`]s unique ID
	fn id(&self) -> ComponentId;

	/// Set the [`Component`]s unique ID
	fn set_id(&mut self, id: ComponentId);

	/// Add a sub [`Activity`]
	fn add_activity(&mut self, activity: Box<dyn Activity>);

	/// Remove the sub [`Activity`] with the given `id`
	fn remove_activity(&mut self, id: ActivityId);

	/// Read access to activities
	/// @TODO: should return an Iterator
	fn activities(&self) -> &Vec<Box<dyn Activity>>;

	/// Write access to activities
	/// @TODO: should return an Iterator
	fn activities_mut(&mut self) -> &mut Vec<Box<dyn Activity>>;

	/// Add a sub [`Component`]
	fn add_component(&mut self, component: Box<dyn Component>);

	/// Remove the sub [`Component`] with the given `id`
	fn remove_component(&mut self, id: ComponentId);

	/// Read access to sub components
	/// @TODO: should return an Iterator
	fn components(&self) -> &Vec<Box<dyn Component>>;

	/// Write access to sub components
	/// @TODO: should return an Iterator
	fn components_mut(&mut self) -> &mut Vec<Box<dyn Component>>;
}
// endregion:   --- Component
