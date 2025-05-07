// Copyright © 2025 Stephan Kunz

//! Implementation for using a tree hierarchy of [`Blackboard`]s within `DiMAS`.
//!
//! This separates the hierarchy from the [`Blackboard`] itself, allowing a [`Blackboard`]
//! beeing part of multiple hierarchies without interferences between those.
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	format,
	string::{String, ToString},
	sync::Arc,
};
use core::{
	any::{Any, TypeId},
	fmt::Debug,
	ops::{Deref, DerefMut},
	str::FromStr,
};
use dimas_core::ConstString;
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
#[derive(Clone, Debug)]
pub struct BlackboardNodeRef {
	/// Path of the [`BlackboardNode`] hierarchy.
	path: ConstString,
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

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, has_remapping, autoremap) = self.get_remapping_info(key);

		// try in current Blackboard
		if self.read().current.read().contains(&final_key) {
			return true;
		}

		// if there is a parent try remapping.
		if let Some(parent) = &mut self.write().parent {
			if has_remapping || autoremap {
				return parent.contains(&final_key);
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

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// Try to delete key in current Blackboard
		let a = self
			.write()
			.current
			.write()
			.delete::<T>(&final_key);
		if a.is_ok() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.delete(&parent_key);
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

		// Check for coded value. These are always "remappings" in the current blackboard.
		let value_option = self.read().values.find(key);
		if let Some(value) = value_option {
			return <T as FromStr>::from_str(&value).map_or_else(
				|_| {
					Err(Error::ParsePortValue(
						key.into(),
						format!("{:?}", TypeId::of::<T>()).into(),
					))
				},
				|val| Ok(val),
			);
		};

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// Try to find in current Blackboard
		let a = self.read().current.read().get::<T>(&final_key);
		if a.is_ok() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get(&parent_key);
			}
		}

		Err(Error::NotFoundIn(parent_key, (&*self.path).into()))
	}

	fn get_entry(&self, key: &str) -> Option<Entry> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_entry(key_stripped);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// try to find key in current Blackboard
		let a = self.read().current.read().get_entry(&final_key);
		if a.is_some() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get_entry(&parent_key);
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

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);

		// try to find key in current Blackboard
		let a = self.read().current.read().get::<T>(&final_key);
		if a.is_ok() {
			return self.read().current.write().set(&final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.set(&parent_key, value);
			}
		}

		// If it is not remapped to parent, set it in current `Blackboard`
		self.read().current.write().set(&final_key, value)
	}
}

impl Environment for BlackboardNodeRef {
	fn define_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().define_env(key_stripped, value);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// try to find key in current Blackboard
		let a = self
			.read()
			.current
			.read()
			.get::<ScriptingValue>(&final_key);
		if a.is_ok() {
			return self
				.read()
				.current
				.write()
				.define_env(&final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.define_env(&parent_key, value);
			}
		}

		// if it is not remapped to parent, set it in current `Blackboard`
		self.read()
			.current
			.write()
			.define_env(&final_key, value)
	}

	fn get_env(&self, key: &str) -> Result<ScriptingValue, ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_env(key_stripped);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self.read().current.read().get_env(&final_key);
		if let Ok(val) = a {
			return Ok(val);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get_env(&parent_key);
			}
		}

		Err(ScriptingError::GlobalNotDefined(final_key))
	}

	fn set_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().set_env(key_stripped, value);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let (final_key, _has_remapping, _autoremap) = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self
			.read()
			.current
			.read()
			.get::<ScriptingValue>(&final_key);
		if a.is_ok() {
			return self
				.read()
				.current
				.write()
				.set_env(&final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.set_env(&parent_key, value);
			}
		}

		Err(ScriptingError::GlobalNotDefined(final_key))
	}
}

impl BlackboardNodeRef {
	/// Create a `BlackboardNodeRef` with remappings and an initial path.
	#[must_use]
	pub fn new(
		path: &str,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		let node = BlackboardNode::new(remappings, values, autoremap);
		Self {
			path: path.into(),
			node: Arc::new(RwLock::new(node)),
		}
	}

	/// Create a `BlackboardNodeRef` with parent and a path extension.
	#[must_use]
	pub fn with(
		path_extension: &str,
		parent: Self,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		let path = String::from(parent.path.clone()) + "/" + path_extension;
		Self {
			path: path.into(),
			node: Arc::new(RwLock::new(BlackboardNode::with(
				parent, remappings, values, autoremap,
			))),
		}
	}

	/// Create a cloned `BlackboardNodeRef`.
	#[must_use]
	pub fn cloned(
		&self,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		let clone = self
			.node
			.read()
			.cloned(remappings, values, autoremap);
		Self {
			path: self.path.clone(),
			node: Arc::new(RwLock::new(clone)),
		}
	}

	/// Print the content of the `BlackboardNodeRef` for debugging purpose
	#[cfg(feature = "std")]
	pub fn debug_message(&self) {
		std::println!("{self:?}");
	}

	/// Read needed remapping information.
	fn get_remapping_info(&self, key: &str) -> (ConstString, bool, bool) {
		let guard = self.read();
		let (remapped_key, has_remapping) = guard
			.remappings
			.find(key)
			.map_or_else(|| (key.into(), false), |remapped| (remapped, true));
		let autoremap = guard.autoremap;
		drop(guard);
		(remapped_key, has_remapping, autoremap)
	}

	/// Read needed remapping information to parent.
	fn get_parent_remapping_info(&self, key: &str) -> (ConstString, bool, bool) {
		let guard = self.read();
		let (remapped_key, has_remapping) = guard.parent_remappings.as_ref().map_or_else(
			|| (key.into(), false),
			|remappings| {
				let (remapped_key, has_remapping) = remappings
					.find(key)
					.map_or_else(|| (key.into(), false), |remapped| (remapped, true));
				(remapped_key, has_remapping)
			},
		);
		let autoremap = guard.parent_autoremap;
		drop(guard);
		(remapped_key, has_remapping, autoremap)
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
	/// Optional [`BlackboardNodeRef`] to a parent [`BlackboardNode`].
	parent: Option<BlackboardNodeRef>,
	/// Optional remappings to the parent.
	parent_remappings: Option<Arc<PortRemappings>>,
	/// Optional parent autoremapping.
	parent_autoremap: bool,
	/// List of [`Port`] remappings.
	remappings: PortRemappings,
	/// List of port values
	values: PortRemappings,
	/// Enables automatic remapping of parent Blackboards ports/keys.
	autoremap: bool,
}
impl BlackboardNode {
	/// Create a new [`BlackboardNode`] with remappings.
	#[must_use]
	pub fn new(remappings: PortRemappings, values: PortRemappings, autoremap: bool) -> Self {
		Self {
			current: BlackboardRef::default(),
			parent: None,
			parent_remappings: None,
			parent_autoremap: false,
			remappings,
			values,
			autoremap,
		}
	}

	/// Create a new [`BlackboardNode`] with parent [`BlackboardNodeRef`].
	/// In that case the remappings are against parent.
	#[must_use]
	pub fn with(
		parent: BlackboardNodeRef,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		Self {
			current: BlackboardRef::default(),
			parent: Some(parent),
			parent_remappings: Some(Arc::new(remappings)),
			parent_autoremap: autoremap,
			remappings: PortRemappings::default(),
			values,
			autoremap: false,
		}
	}

	/// Create a cloned [`BlackboardNode`].
	/// This uses the same [`Blackboard`] and parent [`BlackboardNodeRef`] but own remappings.
	#[must_use]
	pub fn cloned(
		&self,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		Self {
			current: self.current.clone(),
			parent: self.parent.clone(),
			parent_remappings: self.parent_remappings.clone(),
			parent_autoremap: self.parent_autoremap,
			remappings,
			values,
			autoremap,
		}
	}
}
// endregion:   --- BlackboardNode
