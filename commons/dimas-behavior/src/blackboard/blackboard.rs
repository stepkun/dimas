// Copyright © 2024 Stephan Kunz

//! Blackboard of `DiMAS`

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	string::{String, ToString},
	sync::Arc,
};
use core::{
	any::Any,
	ops::{Deref, DerefMut},
	str::FromStr,
};
use dimas_core::value::Value;
use dimas_scripting::{Environment, execution::Error};
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};

use super::ParseStr;
// endregion:   --- modules

// region:      --- types
type EntryPtr = Arc<Mutex<Entry>>;
// endregion:   --- types

// region:      --- Blackboard
/// @TODO:
#[derive(Debug, Default, Clone)]
pub struct Blackboard {
	data: Arc<RwLock<BlackboardData>>,
	parent: Option<Box<Blackboard>>,
}

extern crate std;

impl Environment for Blackboard {
	fn define_env(&self, name: &str, value: Value) {
		match value {
			Value::Nil() => todo!(),
			Value::Boolean(b) => self.set(name, b),
			Value::Float64(f) => self.set(name, f),
			Value::Int64(i) => self.set(name, i),
			Value::String(s) => self.set(name, s),
		}
	}

	fn get_env(&self, name: &str) -> Result<Value, Error> {
		self.get_entry(name).map_or_else(
			|| Err(Error::GlobalNotDefined(name.to_string())),
			|entry| {
				let entry = &(*entry.lock());
				entry.downcast_ref::<String>().map_or_else(
					|| {
						entry.downcast_ref::<f64>().map_or_else(
							|| {
								entry.downcast_ref::<i64>().map_or_else(
									|| {
										entry.downcast_ref::<bool>().map_or_else(
											|| Err(Error::GlobalHasUnknownType(name.to_string())),
											|b| Ok(Value::Boolean(b.to_owned())),
										)
									},
									|i| Ok(Value::Int64(i.to_owned())),
								)
							},
							|f| Ok(Value::Float64(f.to_owned())),
						)
					},
					|s| Ok(Value::String(s.to_owned())),
				)
			},
		)
	}

	fn set_env(&self, name: &str, value: Value) -> Result<(), Error> {
		if self.get_entry(name).is_some() {
			match value {
				Value::Nil() => todo!(),
				Value::Boolean(b) => self.set(name, b),
				Value::Float64(f) => self.set(name, f),
				Value::Int64(i) => self.set(name, i),
				Value::String(s) => self.set(name, s),
			}
			Ok(())
		} else {
			Err(Error::GlobalNotDefined(name.to_string()))
		}
	}
}

impl Blackboard {
	/// Create [`Blackboard`] with a `parent` [`Blackboard`]
	#[must_use]
	pub fn new(parent: &Self) -> Self {
		Self {
			parent: Some(Box::new(parent.clone())),
			..Default::default()
		}
	}

	/// Adds remapping rule for Blackboard. Maps from `internal` (this Blackboard)
	/// to `external` (a parent Blackboard)
	pub fn add_subtree_remapping(&mut self, internal: String, external: String) {
		self.data
			.write()
			.internal_to_external
			.insert(internal, external);
	}

	/// Enables the Blackboard to use autoremapping when getting values from
	/// the parent Blackboard. Only uses autoremapping if there's no matching
	/// explicit remapping rule.
	pub fn enable_auto_remapping(&mut self, use_remapping: bool) {
		self.data.write().auto_remapping = use_remapping;
	}

	/// Version of `get<T>` that does _not_ try to convert from string if the type
	/// doesn't match. This method has the benefit of not requiring the trait
	/// '[`From`] for [`str`]', which allows to avoid implementing the trait for
	/// types that don't need it or it is  not possible to represent the data
	/// type as a string.
	/// @ TODO:
	pub fn get<T>(&self, key: impl AsRef<str>) -> Option<T>
	where
		T: Any + Clone + Send + Sync,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.as_ref().strip_prefix('@') {
			return self.root().get(key_stripped);
		}
		self.get_typed(key.as_ref())
	}

	/// Tries to return the value at `key`. The type `T` must implement
	/// [`FromStr`] when calling this method; it will try to convert
	/// from `String`/`&str` if there's an entry at `key` but it is not
	/// of type `T`. If it does convert it successfully, it will replace
	/// the existing value with `T` so converting from the string type
	/// won't be needed next time.
	///
	/// If you want to get an entry that has a type that doesn't implement
	/// [`FromStr`], use `get_exact<T>` instead.
	/// @ TODO:
	pub fn get_stringy<T>(&self, key: impl AsRef<str>) -> Option<T>
	where
		T: Any + Clone + FromStr + Send + Sync,
	{
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.as_ref().strip_prefix('@') {
			return self.root().get(key_stripped);
		}

		// Try without parsing string first, then try with parsing string
		self.get_typed(key.as_ref())
			.or_else(|| self.__get_allow_string(key.as_ref()))
	}

	/// function to get access to the root blackboard
	/// of a blackboard tree in a recursive way
	fn root(&self) -> Self {
		self.parent
			.clone()
			.map_or_else(|| self.clone(), |bb| bb.root())
	}

	/// @ TODO:
	pub fn set<T: Any + Send + Sync + 'static>(&self, key: impl AsRef<str>, value: T) {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.as_ref().strip_prefix('@') {
			return self.root().set(key_stripped, value);
		}

		self.update_or_create_entry(key.as_ref(), Box::new(value));
	}

	/// Updates the value at `key`, or creates a new [`Entry`].
	fn update_or_create_entry(&self, key: &str, value: Box<dyn Any + Send>) {
		let mut blackboard = self.data.write();

		// If the entry already exists
		if let Some(existing_entry) = blackboard.storage.get(key) {
			existing_entry.lock().0 = value;
		} else if let Some(parent) = self.parent.as_ref() {
			// Use explicit remapping rule
			if let Some(remapped_key) = blackboard.internal_to_external.get(key) {
				parent.update_or_create_entry(remapped_key, value);
			}
			// Use autoremapping
			else if blackboard.auto_remapping {
				parent.update_or_create_entry(key, value);
			}
			// No remapping
			else {
				// Create a new entry
				let entry = Arc::new(Mutex::new(Entry(value)));

				blackboard
					.storage
					.insert(key.to_string(), Arc::clone(&entry));
			}
		}
		// No parent blackboard
		else {
			// Create a new entry
			let entry = Arc::new(Mutex::new(Entry(value)));

			blackboard
				.storage
				.insert(key.to_string(), Arc::clone(&entry));
		}
	}

	/// Get an Rc to the Entry
	#[allow(clippy::significant_drop_tightening)]
	fn get_entry<'a>(&'a self, key: &'a str) -> Option<EntryPtr> {
		// if it is a key starting with an '@' redirect to root bb
		if let Some(key_stripped) = key.strip_prefix('@') {
			return self.root().get_entry(key_stripped);
		}

		let blackboard = self.data.read();

		// Try to get the key
		if let Some(entry) = blackboard.storage.get(key) {
			return Some(Arc::clone(entry));
		}
		// Couldn't find key. Try remapping if we have a parent
		else if let Some(parent_bb) = self.parent.as_ref() {
			// Exists a manual remapping?
			if let Some(remapped_key) = blackboard.internal_to_external.get(key) {
				let parent_entry = parent_bb.get_entry(remapped_key);

				// some optimization by writing a reference to the remapped value into this board
				/*
				if let Some(value) = &parent_entry {
					blackboard
						.storage
						.insert(key.to_string(), Arc::clone(value));
				}
				 */

				return parent_entry;
			}
			// Use auto remapping
			else if blackboard.auto_remapping {
				// Return the value of the parent's `get()`
				return parent_bb.get_entry(key);
			}
		}

		// No matches
		None
	}

	/// Internal method that just tries to get value at key. If the stored
	/// type is not T, return None
	fn get_typed<T>(&self, key: &str) -> Option<T>
	where
		T: Any + Clone,
	{
		self.get_entry(key).and_then(|entry| {
			let entry = entry.lock();

			entry.downcast_ref::<T>().cloned()
		})
	}

	/// Internal method that tries to get the value at key, but only works
	/// if it's a String/&str, then tries [`FromStr`] to convert it to T.
	/// Treats the [`Entry`] as [`Entry::Generic`]
	fn __get_allow_string<T>(&self, key: &str) -> Option<T>
	where
		T: Any + Clone + FromStr + Send,
	{
		// Try to get the key
		if let Some(entry) = self.get_entry(key) {
			let value = self.__get_string(key)?;

			// Try to parse String into T
			if let Ok(value) = <String as ParseStr<T>>::parse_str(&value) {
				// Update value with the value type instead of just a string
				// Because this is the non-`stringy` function, we have to update it as a
				// `Generic`
				*entry.lock() = Entry(Box::new(value.clone()));
				return Some(value);
			}
		}

		// No matches
		None
	}

	/// Internal method that tries to get the value at key as a
	/// `String` or `&str`, returning an owned type
	fn __get_string(&self, key: &str) -> Option<String> {
		self.get_entry(key).and_then(|entry| {
			let entry_lock = entry.lock();
			// If value is a String or &str, try to call [`FromStr`] to convert to T
			entry_lock
				.downcast_ref::<String>()
				.map(ToString::to_string)
				.or_else(|| {
					entry_lock
						.downcast_ref::<&str>()
						.map(ToString::to_string)
				})
		})
	}
}
// endregion:   --- Blackboard

// region:      --- BlackboardData
/// @TODO:
#[derive(Debug, Default)]
struct BlackboardData {
	storage: HashMap<String, Arc<Mutex<Entry>>>,
	internal_to_external: HashMap<String, String>,
	auto_remapping: bool,
}
// endregion:   --- BlackboardData

// region:      --- Entry
/// @TODO:
#[derive(Debug)]
pub struct Entry(Box<dyn Any + Send>);

impl Deref for Entry {
	type Target = Box<dyn Any + Send>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Entry {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
// endregion:   --- Entry
