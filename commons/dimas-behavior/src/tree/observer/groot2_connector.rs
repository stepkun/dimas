// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! [`Groot2Connector`] implementation.
//!

extern crate std;

// region:      --- modules

use dimas_core::ConstString;

use crate::tree::BehaviorTree;
// endregion:   --- modules

// region:      --- Groot2Publisher
/// An observer collecting [`BehaviorTree`] statistics
pub struct Groot2Connector<'a> {
	root: &'a BehaviorTree,
	port: u16,
}

impl<'a> Groot2Connector<'a> {
	/// Construct a new [`Groot2Connector`].
	#[must_use]
	pub const fn new(root: &'a BehaviorTree, port: u16) -> Self {
		Self { root, port }
	}
}
// endregion:   --- Groot2Publisher
