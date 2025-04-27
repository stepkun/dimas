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
use alloc::{borrow::ToOwned, sync::Arc, vec::Vec};
use dimas_core::ConstString;
use libloading::Library;
use parking_lot::Mutex;

use crate::{behavior::{BehaviorCreationFn, BehaviorPtr, BehaviorType}, tree::{BehaviorSubTree, BehaviorTreeComponent, TreeElement}};

use super::error::Error;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for [`Behavior`]s used by the [`BehaviorTreeFactory`] for creation of [`BehaviorTree`]s
#[derive(Default)]
pub struct BehaviorRegistry {
	/// Indicates tat the registry is properly setup, 
	/// i.e. contains all necessary subtrees and the subtrees are linked together.
	is_clean: bool,
	/// List of availabble behaviors.
	behaviors: Vec<(ConstString, BehaviorType, Arc<BehaviorCreationFn>)>,
	/// List of available subtrees.
	subtrees: Vec<BehaviorSubTree>,
	/// List of loaded libraries.
	/// Must be kept in storage until end of programm.
	libraries: Vec<Arc<Library>>,
}

impl BehaviorRegistry {
	/// Add a behavior to the registry
	/// # Errors
	/// - if the entry alreeady exists
	pub fn add_behavior<F>(
		&mut self,
		name: &str,
		bhvr_creation_fn: F,
		bhvr_type: BehaviorType,
	) -> Result<(), Error>
	where
		F: Fn() -> BehaviorPtr + Send + Sync + 'static,
	{
		if self.contains_behavior(name) {
			return Err(Error::BehaviorAlreadyRegistered(name.into()));
		}
		self.behaviors
			.push((name.into(), bhvr_type, Arc::from(bhvr_creation_fn)));
		Ok(())
	}

	/// The Library must be kept in storage until the [`BehaviorTree`] is destroyed.
	/// Therefore the library is stored in the behavior registry, which is later owned by tree.
	/// The `add_library(..)` function also takes care of registering all 'symbols'.
	pub fn add_library(&mut self, library: Library) {
		self.libraries.push(Arc::new(library));
	}

	/// Add a subtree to the registy.
	/// Adding something to the subtree makes subtree 'dirty'.
	pub(crate) fn add_subtree(&mut self, subtree: TreeElement) -> Result<(), Error> {
		for item in &self.subtrees {
			if item.lock().id() == subtree.id() {
				return Err(Error::SubtreeAlreadyRegistered(subtree.id().into()));
			}
		}
		self.subtrees.push(Arc::new(Mutex::new(subtree)));
		self.is_clean = false;
		Ok(())
	}

	/// Check whether registry contains a behavior.
	fn contains_behavior(&self, id: &str) -> bool {
		for (name, _, _) in &self.behaviors {
			if name.as_ref() == id {
				return true;
			}
		}
		false
	}

	/// Fetch a behavior creation function from the registry
	/// # Errors
	/// - if the behavior is not found in the registry
	pub fn fetch(&self, id: &str) -> Result<(BehaviorType, Arc<BehaviorCreationFn>), Error> {
		for (name, bhvr_type, creation_fn) in &self.behaviors {
			if name.as_ref() == id {
				return Ok((bhvr_type.to_owned(), creation_fn.clone()));
			}
		}

		Err(Error::BehaviorNotRegistered(id.into()))
	}
	
	pub(crate) fn link_subtrees(&mut self) -> Result<(), Error> {
		if !self.is_clean {
			for subtree in self.subtrees.clone() {
				self.link_subtree(subtree)?;
			}
			self.is_clean = true;	
		}
		Ok(())
	}

	/// Link each Proxy in a subtree to its subtree
	#[allow(clippy::needless_pass_by_value)]
	fn link_subtree(&self, subtree: BehaviorSubTree) -> Result<(), Error> {
		let node = &mut *subtree.lock();
		for child in &mut node.children_mut().0 {
			self.recursive_node(child)?;
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	#[allow(clippy::needless_pass_by_value)]
	#[allow(clippy::unused_self)]
	#[allow(clippy::match_bool)]
	#[allow(clippy::single_match_else)]
	#[allow(unsafe_code)]
	fn recursive_node(&self, node: &mut TreeElement) -> Result<(), Error> {
		match node {
			TreeElement::Leaf(_leaf) => {},
			TreeElement::Node(node) => {
				for child in &mut node.children_mut().0 {
					self.recursive_node(child)?;
				}
			},
			TreeElement::Proxy(proxy) => {
				let id = proxy.id();
				let subtree = self.subtree_by_name(id)?;
				proxy.set_subtree(subtree);
			},
		}
		Ok(())
	}
	
	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		for (key, _, _) in &self.behaviors {
			std::println!("{key}");
		}
		std::println!();
	}
	
	/// Get aaa reference to the registered libraries
	#[must_use]
	pub(crate) const fn libraries(&self) -> &Vec<Arc<Library>> {
		&self.libraries
	}
	
	/// Get the name list of registered (sub)trees
	#[must_use]
	pub fn registered_behavior_trees(&self) -> Vec<ConstString> {
		let mut res = Vec::new();
		for subtree in &self.subtrees {
			res.push(subtree.lock().id().into());
		}
		res
	}

	/// Find a subtree in the list and return a reference to it
	/// # Errors
	/// - if subtree is not found
	pub fn subtree_by_name(&self, id: &str) -> Result<BehaviorSubTree, Error> {
		for subtree in &self.subtrees {
			// if we are working on a subtree this would become a deadlock
			if let Some(intern) = subtree.try_lock() {
				if intern.id() == id {
					return Ok(subtree.clone());
				}
			}
		}
		Err(Error::SubtreeNotFound(id.into()))
	}
}
// endregion:   --- BehaviorRegistry
