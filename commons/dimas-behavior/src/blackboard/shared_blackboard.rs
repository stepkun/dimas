// Copyright © 2025 Stephan Kunz

//! Implementation for shared usage of [`BlackboardNode`]s within `DiMAS`.
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
use dimas_scripting::{Environment, Error as ScriptingError, execution::ScriptingValue};
use parking_lot::RwLock;

use super::{BlackboardInterface, blackboard::Blackboard, blackboard_data::Entry, error::Error};

use crate::port::PortRemappings;
// endregion:   --- modules

// region:      --- SharedBlackboard
/// Thread safe reference to a [`Blackboard`].
#[derive(Clone, Debug)]
pub struct SharedBlackboard {
	/// Hierarchy of this shared reference.
	path: ConstString,
	/// Shared reference to the [`Blackboard`]
	node: Arc<RwLock<Blackboard>>,
}

impl Deref for SharedBlackboard {
	type Target = Arc<RwLock<Blackboard>>;

	fn deref(&self) -> &Self::Target {
		&self.node
	}
}

impl DerefMut for SharedBlackboard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.node
	}
}

impl BlackboardInterface for SharedBlackboard {
	fn contains(&self, key: ConstString) -> bool {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().contains(key_stripped.into());
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try in current Blackboard
		if self
			.read()
			.content
			.read()
			.contains(final_key.clone())
		{
			return true;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.contains(parent_key);
			}
		}

		false
	}

	fn delete<T>(&mut self, key: ConstString) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().delete(key_stripped.into());
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key.clone());
		// Try to delete key in current Blackboard
		let a = self
			.write()
			.content
			.write()
			.delete::<T>(final_key.clone());
		if a.is_ok() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.delete(parent_key);
			}
		}

		Err(Error::NotFound(key))
	}

	fn get<T>(&self, key: ConstString) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get(key_stripped.into());
		}

		// Check for coded value. These are always "remappings" in the current blackboard.
		let value_option = self.read().values.find(&key);
		if let Some(value) = value_option {
			return <T as FromStr>::from_str(&value).map_or_else(
				|_| {
					Err(Error::ParsePortValue(
						key,
						format!("{:?}", TypeId::of::<T>()).into(),
					))
				},
				|val| Ok(val),
			);
		};

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try to find in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get::<T>(final_key.clone());
		if a.is_ok() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get(parent_key);
			}
		}

		Err(Error::NotFoundIn(parent_key, (&*self.path).into()))
	}

	fn get_entry(&self, key: ConstString) -> Option<Entry> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_entry(key_stripped.into());
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// try to find key in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get_entry(final_key.clone());
		if a.is_some() {
			return a;
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get_entry(parent_key);
			}
		}

		None
	}

	fn set<T>(&mut self, key: ConstString, value: T) -> Result<Option<T>, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().set(key_stripped.into(), value);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get::<T>(final_key.clone());
		if a.is_ok() {
			return self.read().content.write().set(final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.set(parent_key, value);
			}
		}

		// If it is not remapped to parent, set it in current `Blackboard`
		self.read().content.write().set(final_key, value)
	}
}

impl Environment for SharedBlackboard {
	fn define_env(
		&mut self,
		key: ConstString,
		value: ScriptingValue,
	) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().define_env(key_stripped.into(), value);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get::<ScriptingValue>(final_key.clone());
		if a.is_ok() {
			return self
				.read()
				.content
				.write()
				.define_env(final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.define_env(parent_key, value);
			}
		}

		// if it is not remapped to parent, set it in current `Blackboard`
		self.read()
			.content
			.write()
			.define_env(final_key, value)
	}

	fn get_env(&self, key: ConstString) -> Result<ScriptingValue, ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_env(key_stripped.into());
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get_env(final_key.clone());
		if let Ok(val) = a {
			return Ok(val);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &self.read().parent {
				return parent.get_env(parent_key);
			}
		}

		Err(ScriptingError::GlobalNotDefined(final_key))
	}

	fn set_env(&mut self, key: ConstString, value: ScriptingValue) -> Result<(), ScriptingError> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().set_env(key_stripped.into(), value);
		}

		// Read needed remapping values beforehand to avoid a deadlock.
		let final_key = self.get_remapping_info(key);
		// Try to find key in current Blackboard
		let a = self
			.read()
			.content
			.read()
			.get::<ScriptingValue>(final_key.clone());
		if a.is_ok() {
			return self
				.read()
				.content
				.write()
				.set_env(final_key, value);
		}

		// Try to find in parent hierarchy. Again we need to read the remapping info beforehand to avoid deadlocks.
		let (parent_key, has_remapping, autoremap) = self.get_parent_remapping_info(&final_key);
		if has_remapping || autoremap {
			if let Some(parent) = &mut self.write().parent {
				return parent.set_env(parent_key, value);
			}
		}

		Err(ScriptingError::GlobalNotDefined(final_key))
	}
}

impl SharedBlackboard {
	/// Create a `SharedBlackboard` with remappings and an initial path.
	#[must_use]
	pub fn new(creator: ConstString, remappings: PortRemappings, values: PortRemappings) -> Self {
		let node = Blackboard::new(creator.clone(), remappings, values);
		Self {
			path: creator,
			node: Arc::new(RwLock::new(node)),
		}
	}

	/// Create a `SharedBlackboard` with parent and a path extension.
	#[must_use]
	pub fn with(
		creator: ConstString,
		parent: Self,
		remappings: PortRemappings,
		values: PortRemappings,
		autoremap: bool,
	) -> Self {
		let path = String::from(&*parent.path) + "/" + &creator;
		Self {
			path: path.into(),
			node: Arc::new(RwLock::new(Blackboard::with(
				creator, parent, remappings, values, autoremap,
			))),
		}
	}

	/// Create a cloned `SharedBlackboard`.
	#[must_use]
	pub fn cloned(&self, remappings: PortRemappings, values: PortRemappings) -> Self {
		let clone = self.node.read().cloned(remappings, values);
		Self {
			path: self.path.clone(),
			node: Arc::new(RwLock::new(clone)),
		}
	}

	/// Print the content of the `SharedBlackboard` for debugging purpose.
	#[cfg(feature = "std")]
	pub fn debug_message(&self) {
		std::println!("{self:?}");
	}

	/// Read needed remapping information.
	fn get_remapping_info(&self, key: ConstString) -> ConstString {
		let guard = self.read();
		let remapped_key = guard
			.remappings
			.find(key.as_ref())
			.map_or_else(|| key, |remapped| remapped);
		drop(guard);
		remapped_key
	}

	/// Read needed remapping information to parent.
	fn get_parent_remapping_info(&self, key: &ConstString) -> (ConstString, bool, bool) {
		let guard = self.read();
		let (remapped_key, has_remapping) = guard.remappings_to_parent.as_ref().map_or_else(
			|| (key.clone(), false),
			|remappings| {
				let (remapped_key, has_remapping) = remappings
					.find(key.as_ref())
					.map_or_else(|| (key.clone(), false), |remapped| (remapped, true));
				(remapped_key, has_remapping)
			},
		);
		let autoremap = guard.autoremap_to_parent;
		drop(guard);
		(remapped_key, has_remapping, autoremap)
	}

	/// function to get access to the root [`BlackboardNode`]
	/// of a [`Blackboard`] tree in a recursive way.
	fn root(&self) -> Self {
		self.node
			.read()
			.parent
			.as_ref()
			.map_or_else(|| self.clone(), Self::root)
	}

	/// Add or change the parent of a [`SharedBlackboard`].
	pub fn set_parent(&self, parent: Self) {
		self.write().parent = Some(parent);
	}
}
// endregion:   --- SharedBlackboard
