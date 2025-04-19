// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]

//! [`BehaviorRegistry`] library
//!
//! The registry is not using a `HashMap` but a `Vec` due to two reasons:
//! - A `HashMap` needs more space than a `Vec` and search performance is not an issue
//! - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{borrow::ToOwned, boxed::Box, string::String, sync::Arc, vec::Vec};
use libloading::Library;

use crate::new_behavior::{BehaviorCreationFn, BehaviorTreeMethods, NewBehaviorType};

use super::error::Error;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for [`Behavior`]s used by the [`BehaviorTreeFactory`] for creation of [`BehaviorTree`]s
#[derive(Default)]
pub struct BehaviorRegistry {
	behaviors: Vec<(String, NewBehaviorType, Arc<BehaviorCreationFn>)>,
	librarys: Vec<Library>,
}

impl BehaviorRegistry {
	/// Add a behavior to the registry
	/// # Errors
	/// - if the entry alreeady exists
	pub fn add_behavior<F>(
		&mut self,
		name: impl Into<String>,
		bhvr_creation_fn: F,
		bhvr_type: NewBehaviorType,
	) -> Result<(), Error>
	where
		F: Fn() -> Box<dyn BehaviorTreeMethods> + Send + Sync + 'static,
	{
		let name = name.into();
		if self.contains(&name) {
			return Err(Error::BehaviorAlreadyRegistered(name));
		}
		self.behaviors
			.push((name, bhvr_type, Arc::from(bhvr_creation_fn)));
		Ok(())
	}

	/// The Library must be kept in storage until the [`BehaviorTree`] is destroyed.
	/// Therefore the library is stored in the behavior registry, which is later owned by tree.
	/// The `add_library(..)` function also takes care of registering all 'symbols'.
	pub fn add_library(&mut self, library: Library) {
		self.librarys.push(library);
	}

	/// Register a behavior from a `dylib` in the registry
	/// # Errors
	/// - if the entry alreeady exists
	pub extern "Rust" fn register_behavior(
		&mut self,
		name: &str,
		bhvr_creation_fn: Box<dyn Fn() -> Box<dyn BehaviorTreeMethods> + Send + Sync + 'static>,
		bhvr_type: NewBehaviorType,
	) -> Result<(), Error> {
		if self.contains(name) {
			return Err(Error::BehaviorAlreadyRegistered(name.into()));
		}
		self.behaviors
			.push((name.into(), bhvr_type, Arc::from(bhvr_creation_fn)));
		Ok(())
	}

	/// Check whether registry contains an entry.
	fn contains(&self, id: &str) -> bool {
		for (name, _, _) in &self.behaviors {
			if name == id {
				return true;
			}
		}

		false
	}

	/// Fetch a behavior creation function from the registry
	/// # Errors
	/// - if the behavior is not found in the registry
	pub fn fetch(&self, id: &str) -> Result<(NewBehaviorType, Arc<BehaviorCreationFn>), Error> {
		for (name, bhvr_type, creation_fn) in &self.behaviors {
			if name == id {
				return Ok((bhvr_type.to_owned(), creation_fn.clone()));
			}
		}

		Err(Error::BehaviorNotRegistered(id.into()))
	}

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		for (key, _, _) in &self.behaviors {
			std::println!("{key}");
		}
		std::println!();
	}
}
// endregion:   --- BehaviorRegistry
