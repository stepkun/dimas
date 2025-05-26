// Copyright © 2025 Stephan Kunz

//! Implementation for using a tree hierarchy of [`Blackboard`]s within `DiMAS`.
//!
//! This separates the hierarchy from the [`Blackboard`] itself, allowing a [`Blackboard`]
//! beeing part of multiple hierarchies without interferences between those.
//!

// region:      --- modules
use alloc::sync::Arc;
use core::fmt::Debug;
use dimas_core::ConstString;
use parking_lot::RwLock;

use super::{BlackboardData, SharedBlackboard};

use crate::port::PortRemappings;
// endregion:   --- modules

// region:      --- Blackboard
/// Implementation of a [`Blackboard`] with a possible parent, a [`SharedBlackboard`],
/// internal & external remappings and/or value assignments, all as [`PortRemappings`].
/// 
/// Access to the fields is public within this module.
#[derive(Debug, Default)]
pub struct Blackboard {
	/// Creator of the Blackboard.
	pub(super) creator: ConstString,
	/// Reference to the managed [`BlackboardData`].
	pub(super) content: Arc<RwLock<BlackboardData>>,
	/// Optional parent [`SharedBlackboard`].
	pub(super) parent: Option<SharedBlackboard>,
	/// Optional lsit of [`PortRemappings`] to the parent.
	pub(super) remappings_to_parent: Option<Arc<PortRemappings>>,
	/// Optional autoremapping to the parent.
	pub(super) autoremap_to_parent: bool,
	/// List of internal [`PortRemappings`].
	pub(super) remappings: PortRemappings,
	/// List of direct assigned values to a `Port`.
	pub(super) values: PortRemappings,
}
impl Blackboard {
	/// Create a new [`Blackboard`] with remappings.
	#[must_use]
	pub fn new(creator: ConstString, remappings: PortRemappings, values: PortRemappings) -> Self {
		Self {
			creator,
			content: Arc::new(RwLock::new(BlackboardData::default())),
			parent: None,
			remappings_to_parent: None,
			autoremap_to_parent: false,
			remappings,
			values,
		}
	}

	/// Create a new [`Blackboard`] with parent [`SharedBlackboard`].
	/// In that case the remappings are against parent.
	#[must_use]
	pub fn with(
		creator: ConstString,
		parent: SharedBlackboard,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		Self {
			creator,
			content: Arc::new(RwLock::new(BlackboardData::default())),
			parent: Some(parent),
			remappings_to_parent: Some(Arc::new(remappings)),
			autoremap_to_parent: autoremap,
			remappings: PortRemappings::default(),
			values,
		}
	}

	/// Create a cloned [`Blackboard`].
	/// This uses the same [`Blackboard`] and parent [`SharedBlackboard`] but own remappings.
	#[must_use]
	pub fn cloned(&self, remappings: PortRemappings, values: PortRemappings) -> Self {
		Self {
			creator: self.creator.clone(),
			content: self.content.clone(),
			parent: self.parent.clone(),
			remappings_to_parent: self.remappings_to_parent.clone(),
			autoremap_to_parent: self.autoremap_to_parent,
			remappings,
			values,
		}
	}
}
// endregion:   --- Blackboard
