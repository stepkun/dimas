// Copyright © 2025 Stephan Kunz

//! Blackboard implementation for `DiMAS`.
//!
//! In `DiMAS` the `Blackboard`s are used in a hierarchical structure, remapping the key names
//! and partly "inheriting" values from parent `Blackboard`s.
//!

#[allow(clippy::module_inception)]
mod blackboard;
mod blackboard_node;
pub mod error;
// mod old_blackboard;
// mod string;
// pub use string::*;

// flatten
pub use blackboard::{Blackboard, BlackboardRef};
pub use blackboard_node::BlackboardNodeRef;

// region:      --- modules
use alloc::string::ToString;
use blackboard::Entry;
use core::{any::Any, fmt::Debug, str::FromStr};

use self::error::Error;
// endregion:   --- modules

// region:      --- BlackboardInterface
/// Contract for interacting with a [`Blackboard`] or a [`BlackboardNode`].
pub trait BlackboardInterface {
	/// Check whether a certain key is within the [`Blackboard`].
	fn contains(&self, key: &str) -> bool;

	/// Delete a value of type T with key from [`Blackboard`].
	/// Return the old value.
	/// # Errors
	/// - if key is not in [`Blackboard`]
	/// - if key has different type than expected
	fn delete<T>(&mut self, key: &str) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static;

	/// Get a value of type T with key from [`Blackboard`].
	/// # Errors
	/// - if key is not in [`Blackboard`]
	/// - if key has different type than expected
	fn get<T>(&self, key: &str) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static;

	/// Get raw [`Entry`] with key from [`Blackboard`].
	fn get_entry(&self, key: &str) -> Option<Entry>;

	/// Set a value of type T with key in the [`Blackboard`].
	/// Returns an eventually existing value.
	/// # Errors
	/// - if key already exists with a different type
	fn set<T>(&mut self, key: &str, value: T) -> Result<Option<T>, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static;
}
// endregion:   --- BlackboardInterface
