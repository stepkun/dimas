// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port implementation

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::vec::Vec;
use dimas_core::ConstString;

use super::error::Error;
// endregion:   --- modules

// region:		--- types
/// An immutable remapping entry
type RemappingEntry = (ConstString, ConstString);
// endregion:   --- types

// region:		--- PortRemappings
/// Remapping list
/// `PortRemappings` is not using a `HashMap` but a `Vec` due to two reasons:
/// - A `HashMap` needs more space than a `Vec` and search performance is not an issue
/// - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized
#[derive(Debug, Default)]
pub struct PortRemappings(Vec<RemappingEntry>);

impl PortRemappings {
	/// Add an entry to the [`PortRemappings`]
	/// # Errors
	/// - if entry already exists
	pub fn add(&mut self, name: &str, remapped_name: &str) -> Result<(), Error> {
		for (original, _) in &self.0 {
			if original.as_ref() == name {
				return Err(Error::AlreadyInRemappings(name.into()));
			}
		}
		self.0.push((name.into(), remapped_name.into()));
		Ok(())
	}

	/// Lookup the remapped name.
	#[must_use]
	pub fn find(&self, name: &str) -> Option<ConstString> {
		for (original, remapped) in &self.0 {
			if original.as_ref() == name {
				return Some(remapped.clone());
			}
		}
		None
	}

	/// Optimize for size
	pub fn shrink(&mut self) {
		self.0.shrink_to_fit();
	}
}
// endregion:   --- PortRemappings
