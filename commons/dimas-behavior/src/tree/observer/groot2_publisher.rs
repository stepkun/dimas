// Copyright © 2025 Stephan Kunz

//! [`Groot2Publisher`] implementation.
//!

extern crate std;

// region:      --- modules

use crate::tree::BehaviorTree;
// endregion:   --- modules

// region:      --- Groot2Publisher
/// An observer collecting [`BehaviorTree`] statistics
pub struct Groot2Publisher {}

impl Groot2Publisher {
	/// Construct a new [`Groot2Publisher`].
	#[must_use]
	pub const fn new(_root: &BehaviorTree, _port: i16) -> Self {
		Self {}
	}
}
// endregion:   --- Groot2Publisher
