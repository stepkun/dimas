// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port implementation

#[doc(hidden)]
extern crate alloc;

use core::ops::Deref;

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
/// The `PortRemappings` is not using a `BTreeMap` but a `Vec` due to
/// a `BTreeMap` needs more space than a `Vec` and search performance is not an issue
#[derive(Clone, Debug, Default)]
pub struct PortRemappings(Vec<RemappingEntry>);

impl Deref for PortRemappings {
	type Target = Vec<RemappingEntry>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl PortRemappings {
	/// Add an entry to the [`PortRemappings`].
	/// # Errors
	/// - if entry already exists
	pub fn add(&mut self, name: &ConstString, remapped_name: &ConstString) -> Result<(), Error> {
		for (original, _) in &self.0 {
			if original == name {
				return Err(Error::AlreadyInRemappings(name.clone()));
			}
		}
		self.0.push((name.clone(), remapped_name.clone()));
		Ok(())
	}

	/// Add an entry to the [`PortRemappings`].
	/// Already existing values will be overwritten
	pub fn overwrite(&mut self, key: &ConstString, value: &ConstString) {
		self.0.push((key.clone(), value.clone()));
	}

	/// Lookup the remapped name.
	#[must_use]
	pub fn find(&self, name: &ConstString) -> Option<ConstString> {
		for (original, remapped) in &self.0 {
			if original == name {
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
