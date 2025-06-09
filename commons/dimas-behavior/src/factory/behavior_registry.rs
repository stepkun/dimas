// Copyright © 2025 Stephan Kunz

//! [`BehaviorRegistry`] library
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{collections::btree_map::BTreeMap, sync::Arc, vec::Vec};
use dimas_core::ConstString;
use dimas_scripting::Runtime;
use libloading::Library;

use crate::behavior::{BehaviorCreationFn, BehaviorPtr, BehaviorType};

use super::error::Error;

#[cfg(doc)]
use super::BehaviorTreeFactory;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for behaviors used by the [`BehaviorTreeFactory`] for creation of behavior trees.
#[derive(Default)]
pub struct BehaviorRegistry {
	/// [`BTreeMap`] of available behavior creation functions.
	behaviors: BTreeMap<ConstString, (BehaviorType, Arc<BehaviorCreationFn>)>,
	/// [`BTreeMap`] of registered behavior tree definitions.
	tree_definitions: BTreeMap<ConstString, ConstString>,
	/// Scripting runtime
	runtime: Runtime,
	/// List of loaded libraries.
	/// Every tree must keep a reference to its needed libraries to keep the libraries in memory
	/// until end of programm.
	libraries: Vec<Arc<Library>>,
}

impl BehaviorRegistry {
	/// Add a behavior to the registry
	/// # Errors
	/// - if the entry already exists
	pub fn add_behavior<F>(&mut self, name: &str, bhvr_creation_fn: F, bhvr_type: BehaviorType) -> Result<(), Error>
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
	pub(super) fn add_tree_defintion(&mut self, id: &str, tree_definition: ConstString) -> Result<(), Error> {
		let key: ConstString = id.into();
		if let std::collections::btree_map::Entry::Vacant(e) = self.tree_definitions.entry(key) {
			e.insert(tree_definition);
			Ok(())
		} else {
			Err(Error::SubtreeAlreadyRegistered(id.into()))
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

	pub(super) fn find_tree_definition(&self, name: &str) -> Option<ConstString> {
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
	pub fn registered_behavior_trees(&self) -> Vec<ConstString> {
		let mut res = Vec::new();
		for id in self.tree_definitions.keys() {
			res.push(id.clone());
		}
		res
	}

	/// Access the runtime.
	#[must_use]
	pub const fn runtime(&self) -> &Runtime {
		&self.runtime
	}

	/// Access the runtime mutable.
	pub const fn runtime_mut(&mut self) -> &mut Runtime {
		&mut self.runtime
	}

	pub(crate) fn register_enum_tuple(&mut self, key: &str, value: i8) -> Result<(), Error> {
		self.runtime.register_enum_tuple(key, value)?;
		Ok(())
	}
}
// endregion:   --- BehaviorRegistry
