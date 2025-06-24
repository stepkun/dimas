// Copyright © 2025 Stephan Kunz

//! Implementation for using a tree hierarchy of [`Blackboard`]s within `DiMAS`.
//!
//! This separates the hierarchy from the [`Blackboard`] itself, allowing a [`Blackboard`]
//! beeing part of multiple hierarchies without interferences between those.
//!

// region:      --- modules
use alloc::sync::Arc;
use core::fmt::Debug;
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
	/// Reference to the managed [`BlackboardData`].
	pub(super) content: Arc<RwLock<BlackboardData>>,
	/// Optional parent [`SharedBlackboard`].
	pub(super) parent: Option<SharedBlackboard>,
	/// Optional lsit of [`PortRemappings`] to the parent.
	pub(super) remappings_to_parent: Option<PortRemappings>,
	/// Optional autoremapping to the parent.
	pub(super) autoremap_to_parent: bool,
}

impl Blackboard {
	/// Create a new [`Blackboard`].
	#[must_use]
	pub fn new() -> Self {
		Self {
			content: Arc::new(RwLock::new(BlackboardData::default())),
			parent: None,
			remappings_to_parent: None,
			autoremap_to_parent: false,
		}
	}

	/// Create a new [`Blackboard`] with remappings.
	#[must_use]
	pub fn with(remappings: PortRemappings) -> Self {
		Self {
			content: Arc::new(RwLock::new(BlackboardData::default())),
			parent: None,
			remappings_to_parent: Some(remappings),
			autoremap_to_parent: false,
		}
	}

	/// Create a new [`Blackboard`] with parent [`SharedBlackboard`].
	/// In that case the remappings are against parent.
	#[must_use]
	pub fn with_parent(parent: SharedBlackboard, remappings: PortRemappings, autoremap: bool) -> Self {
		Self {
			content: Arc::new(RwLock::new(BlackboardData::default())),
			parent: Some(parent),
			remappings_to_parent: Some(remappings),
			autoremap_to_parent: autoremap,
		}
	}
}
// endregion:   --- Blackboard
