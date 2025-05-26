// Copyright © 2025 Stephan Kunz

//! [`BehaviorRegistry`] library
//!
//! The registry is not using a `HashMap` but a `Vec` due to two reasons:
//! - A `HashMap` needs more space than a `Vec` and search performance is not an issue
//! - A `HashMap` does not work well with loaded libraries, as the hash seeds must be synchronized

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{sync::Arc, vec::Vec};
use dimas_core::BoxConstString;
use hashbrown::HashMap;
use libloading::Library;
use rustc_hash::FxBuildHasher;

use crate::behavior::{BehaviorCreationFn, BehaviorPtr, BehaviorType};

use super::error::Error;

#[cfg(doc)]
use super::BehaviorTreeFactory;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for behaviors used by the [`BehaviorTreeFactory`] for creation of behavior trees.
#[derive(Default)]
pub struct BehaviorRegistry {
	/// [`HashMap`] of available behavior creation functions.
	behaviors: HashMap<BoxConstString, (BehaviorType, Arc<BehaviorCreationFn>), FxBuildHasher>,
	/// [`HashMap`] of registered behavior tree definitions.
	tree_definitions: HashMap<BoxConstString, BoxConstString, FxBuildHasher>,
	/// List of loaded libraries.
	/// Every tree must keep a reference to its needed libraries to keep the libraries in memory
	/// until end of programm.
	libraries: Vec<Arc<Library>>,
}

impl BehaviorRegistry {
	/// Add a behavior to the registry
	/// # Errors
	/// - if the entry already exists
	pub fn add_behavior<F>(
		&mut self,
		name: &str,
		bhvr_creation_fn: F,
		bhvr_type: BehaviorType,
	) -> Result<(), Error>
	where
		F: Fn() -> BehaviorPtr + Send + Sync + 'static,
	{
		if self.behaviors.contains_key(name) {
			return Err(Error::BehaviorAlreadyRegistered(name.into()));
		}
		self.behaviors
			.insert(name.into(), (bhvr_type, Arc::from(bhvr_creation_fn)));
		Ok(())
	}

	/// The Library must be kept in storage until the behaviort tree is destroyed.
	/// Therefore the library is stored in the behavior registry and later a cloned
	/// reference is handed over to every created tree.
	pub fn add_library(&mut self, library: Library) {
		self.libraries.push(Arc::new(library));
	}

	/// Add a behavior tree definition to the registry.
	/// # Errors
	/// - if the behavior tree definition is already registered.
	pub(super) fn add_tree_defintion(
		&mut self,
		id: &str,
		tree_definition: BoxConstString,
	) -> Result<(), Error> {
		let id: BoxConstString = id.into();
		if self.tree_definitions.contains_key(&id) {
			Err(Error::SubtreeAlreadyRegistered(id.into()))
		} else {
			self.tree_definitions.insert(id, tree_definition);
			Ok(())
		}
	}

	/// Fetch a behavior creation function from the registry.
	/// # Errors
	/// - if the behavior is not found in the registry
	pub fn fetch(&self, id: &str) -> Result<(BehaviorType, Arc<BehaviorCreationFn>), Error> {
		self.behaviors.get(id).map_or_else(
			|| Err(Error::BehaviorNotRegistered(id.into())),
			|value| Ok(value.clone()),
		)
	}

	pub(super) fn find_tree_definition(&self, name: &str) -> Option<BoxConstString> {
		self.tree_definitions.get(name).cloned()
	}

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		let iter = self.behaviors.iter();
		for (key, _) in iter {
			std::println!("{key}");
		}
		std::println!();
	}

	/// Get a reference to the registered libraries
	#[must_use]
	pub(crate) const fn libraries(&self) -> &Vec<Arc<Library>> {
		&self.libraries
	}

	/// Get the name list of registered (sub)trees
	#[must_use]
	pub fn registered_behavior_trees(&self) -> Vec<BoxConstString> {
		let mut res = Vec::new();
		for (id, _) in &self.tree_definitions {
			res.push(id.clone());
		}
		res
	}
}
// endregion:   --- BehaviorRegistry
