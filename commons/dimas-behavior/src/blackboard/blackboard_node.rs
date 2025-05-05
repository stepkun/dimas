// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Node implementation for a tree hierarchy of [`Blackboard`]s within `DiMAS`.
//!
//! This separates the hierarchy from the [`Blackboard`] itself, allowing a [`Blackboard`]
//! beeing part of multiple hierarchies without interferences between those.
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	string::{String, ToString},
	sync::Arc,
};
use core::{
	any::Any,
	fmt::Debug,
	ops::{Deref, DerefMut},
	str::FromStr,
};
use dimas_scripting::{
	Environment,
	execution::{Error as ScriptingError, ScriptingValue},
};
use parking_lot::RwLock;

use super::{BlackboardInterface, BlackboardRef, blackboard::Entry, error::Error};

use crate::port::PortRemappings;
// endregion:   --- modules

// region:      --- BlackboardNodeRef
/// Thread safe reference to a [`BlackboardNode`].
#[derive(Clone, Debug, Default)]
pub struct BlackboardNodeRef {
	node: Arc<RwLock<BlackboardNode>>,
}

impl Deref for BlackboardNodeRef {
	type Target = Arc<RwLock<BlackboardNode>>;

	fn deref(&self) -> &Self::Target {
		&self.node
	}
}

impl DerefMut for BlackboardNodeRef {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.node
	}
}

impl BlackboardInterface for BlackboardNodeRef {
	fn contains(&self, key: &str) -> bool {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().contains(key_stripped);
		}

		// try in current Blackboard
		if self.read().current.read().contains(key) {
			return true;
		}

		// if there is a parent try remapping. Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &mut self.write().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.contains(&remapped_key);
			} else if autoremap {
				return parent.contains(key);
			}
		}

		false
	}

	fn delete<T>(&mut self, key: &str) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().delete(key_stripped);
		}

		// try to delete key in current Blackboard
		let a = self.write().current.write().delete::<T>(key);
		if a.is_ok() {
			return a;
		}

		// if there is a parent try remapping. Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &mut self.write().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.delete(&remapped_key);
			} else if autoremap {
				return parent.delete(key);
			}
		}
		Err(Error::NotFound(key.into()))
	}

	fn get<T>(&self, key: &str) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get(key_stripped);
		}

		// Try to find key in current Blackboard
		let a = self.read().current.read().get::<T>(key);
		if a.is_ok() {
			return a;
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};

		// Try to find remapped key in current Blackboard.
		if let Some(remapped_key) = remapped.clone() {
			let a = self.read().current.read().get::<T>(&remapped_key);
			if a.is_ok() {
				return a;
			}
		}

		// Try to find in parent hierarchy.
		if let Some(parent) = &self.read().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped.clone() {
				return parent.get(&remapped_key);
			} else if autoremap {
				return parent.get(key);
			}
		}

		let search = remapped.map_or_else(
			|| String::from(key),
			|remapped_key| String::from(key) + "/" + &remapped_key,
		);

		Err(Error::NotFound(search.into()))
	}

	fn get_entry(&self, key: &str) -> Option<Entry> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_entry(key_stripped);
		}

		// try to find key in current Blackboard
		let a = self.read().current.read().get_entry(key);
		if a.is_some() {
			return a;
		}

		// if there is a parent try remapping
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &self.read().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.get_entry(&remapped_key);
			} else if autoremap {
				return parent.get_entry(key);
			}
		}
		None
	}

	fn set<T>(&mut self, key: &str, value: T) -> Result<Option<T>, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().set(key_stripped, value);
		}

		// try to find key in current Blackboard
		let a = self.read().current.read().get::<T>(key);
		if a.is_ok() {
			return self.read().current.write().set(key, value);
		}

		// if there is a parent do remapping otherwise create in current Blackboard.
		// Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &mut self.write().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.set(&remapped_key, value);
			} else if autoremap {
				return parent.set(key, value);
			}
		}
		// if it is not remapped, set it in current `Blackboard`
		self.read().current.write().set(key, value)
	}
}

impl Environment for BlackboardNodeRef {
	fn define_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().define_env(key_stripped, value);
		}

		// try to find key in current Blackboard
		let a = self
			.read()
			.current
			.read()
			.get::<ScriptingValue>(key);
		if a.is_ok() {
			return self.read().current.write().define_env(key, value);
		}

		// if there is a parent do remapping otherwise create in current Blackboard.
		// Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &mut self.write().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.define_env(&remapped_key, value);
			} else if autoremap {
				return parent.define_env(key, value);
			}
		}

		// if it is not remapped, set it in current `Blackboard`
		self.read().current.write().define_env(key, value)
	}

	fn get_env(&self, key: &str) -> Result<ScriptingValue, ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_env(key_stripped);
		}

		// try to find key in current Blackboard
		let a = self.read().current.read().get_env(key);
		if let Ok(val) = a {
			return Ok(val);
		}

		// if there is a parent do remapping otherwise create in current Blackboard.
		// Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &self.read().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.get_env(&remapped_key);
			} else if autoremap {
				return parent.get_env(key);
			}
		}

		Err(ScriptingError::GlobalNotDefined(key.into()))
	}

	fn set_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().set_env(key_stripped, value);
		}

		// try to find key in current Blackboard
		let a = self
			.read()
			.current
			.read()
			.get::<ScriptingValue>(key);
		if a.is_ok() {
			return self.read().current.write().set_env(key, value);
		}

		// if there is a parent do remapping
		// Read needed values beforehand to avoid a deadlock.
		let (remapped, autoremap) = {
			let guard = self.read();
			let remapped = guard.remappings.find(key);
			let autoremap = guard.autoremap;
			drop(guard);
			(remapped, autoremap)
		};
		if let Some(parent) = &mut self.write().parent {
			// prefer manual remapping over autoremap
			if let Some(remapped_key) = remapped {
				return parent.set_env(&remapped_key, value);
			} else if autoremap {
				return parent.set_env(key, value);
			}
		}

		Err(ScriptingError::GlobalNotDefined(key.into()))
	}
}

impl BlackboardNodeRef {
	/// Create a `BlackboardNodeRef` with remappings.
	#[must_use]
	pub fn new(remappings: PortRemappings, autoremap: bool) -> Self {
		let node = BlackboardNode::new(remappings, autoremap);
		Self {
			node: Arc::new(RwLock::new(node)),
		}
	}

	/// Create a `BlackboardNodeRef` with parent.
	#[must_use]
	pub fn with(parent: Self, remappings: PortRemappings, autoremap: bool) -> Self {
		Self {
			node: Arc::new(RwLock::new(BlackboardNode::with(
				parent, remappings, autoremap,
			))),
		}
	}

	/// Create a cloned `BlackboardNodeRef`.
	#[must_use]
	pub fn cloned(&self, remappings: PortRemappings, autoremap: bool) -> Self {
		let clone = self.node.read().cloned(remappings, autoremap);
		Self {
			node: Arc::new(RwLock::new(clone)),
		}
	}

	/// Print the content of the `BlackboardNode` for debugging purpose
	#[cfg(feature = "std")]
	pub fn debug_message(&self) {
		std::println!("{self:?}");
	}

	/// function to get access to the root [`BlackboardNode`]
	/// of a [`Blackboard`] tree in a recursive way.
	#[allow(clippy::redundant_closure_for_method_calls)]
	fn root(&self) -> Self {
		self.node
			.read()
			.parent
			.as_ref()
			.map_or_else(|| self.clone(), |bb| bb.root())
	}

	/// Add or change the parent of a [`BlackboardNodeRef`].
	pub fn set_parent(&self, parent: Self) {
		self.write().parent = Some(parent);
	}
}
// endregion:   --- BlackboardNodeRef

// region:      --- BlackboardNode
/// Node implementation for the [`Blackboard`] hierarchy.
#[derive(Debug, Default)]
pub struct BlackboardNode {
	/// Reference to the managed [`Blackboard`].
	current: BlackboardRef,
	/// Optional reference to a parent [`BlackboardNode`].
	parent: Option<BlackboardNodeRef>,
	/// List of [`Port`] remappings.
	remappings: PortRemappings,
	/// Automatic remapping of parent Blackboards ports.
	autoremap: bool,
}
impl BlackboardNode {
	/// Create a `BlackboardNode` with remappings.
	#[must_use]
	pub fn new(remappings: PortRemappings, autoremap: bool) -> Self {
		Self {
			current: BlackboardRef::default(),
			parent: None,
			remappings,
			autoremap,
		}
	}

	/// Create a `BlackboardNode` with parent.
	#[must_use]
	pub fn with(parent: BlackboardNodeRef, remappings: PortRemappings, autoremap: bool) -> Self {
		Self {
			current: BlackboardRef::default(),
			parent: Some(parent),
			remappings,
			autoremap,
		}
	}

	/// Create a cloned `BlackboardNode`.
	#[must_use]
	pub fn cloned(&self, remappings: PortRemappings, autoremap: bool) -> Self {
		Self {
			current: self.current.clone(),
			parent: self.parent.clone(),
			remappings,
			autoremap,
		}
	}
}
// endregion:   --- BlackboardNode
