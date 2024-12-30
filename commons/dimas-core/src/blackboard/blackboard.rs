// Copyright © 2024 Stephan Kunz

//! Blackboard of `DiMAS`

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
	sync::Arc,
};
use core::any::Any;
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};

use super::{AnyStringy, FromString, ParseStr};
// endregion:   --- modules

// region:      --- types
type EntryPtr = Arc<Mutex<Entry>>;
// endregion:   --- types

// region:      --- Blackboard
/// @TODO:
#[derive(Debug, Default, Clone)]
pub struct Blackboard {
	data: Arc<RwLock<BlackboardData>>,
	parent: Box<Option<Blackboard>>,
}

impl Blackboard {
	/// Create [`Blackboard`] with a `parent` [`Blackboard`]
	#[must_use]
	pub fn new(parent: &Self) -> Self {
		Self {
			parent: Box::new(Some(parent.clone())),
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

	/// Tries to return the value at `key`. The type `T` must implement
	/// `FromString` when calling this method; it will try to convert
	/// from `String`/`&str` if there's an entry at `key` but it is not
	/// of type `T`. If it does convert it successfully, it will replace
	/// the existing value with `T` so converting from the string type
	/// won't be needed next time.
	///
	/// If you want to get an entry that has a type that doesn't implement
	/// `FromString`, use `get_exact<T>` instead.
	/// @ TODO:
	pub fn get_stringy<T>(&mut self, key: impl AsRef<str>) -> Option<T>
	where
		T: Any + Clone + FromString + Send + Sync,
	{
		// Try without parsing string first, then try with parsing string
		self.__get_no_string(key.as_ref())
			.or_else(|| self.__get_allow_string(key.as_ref()))
	}

	/// Version of `get<T>` that does _not_ try to convert from string if the type
	/// doesn't match. This method has the benefit of not requiring the trait
	/// `FromString`, which allows you to avoid implementing the trait for
	/// types that don't need it or it's impossible to represent the data
	/// type as a string.
	/// @ TODO:
	pub fn get<T>(&mut self, key: impl AsRef<str>) -> Option<T>
	where
		T: Any + Clone + Send + Sync,
	{
		self.__get_no_string(key.as_ref())
	}

	/// @ TODO:
	pub fn set<T: Any + Send + Sync + 'static>(&mut self, key: impl AsRef<str>, value: T) {
		let key = key.as_ref().to_string();

		let blackboard = self.data.write();

		if let Some(entry) = blackboard.storage.get(&key) {
			let mut entry = entry.lock();

			// Overwrite value of existing entry
			*entry = Entry::Generic(Box::new(value));
		} else {
			drop(blackboard);
			let entry = self.create_entry(&key);

			let mut entry = entry.lock();

			// Set value of new entry
			*entry = Entry::Generic(Box::new(value));
		}
	}

	/// Get an Rc to the Entry
	#[allow(clippy::significant_drop_tightening)]
	fn get_entry<'a>(&'a mut self, key: &'a str) -> Option<EntryPtr> {
		let mut blackboard = self.data.write();

		// Try to get the key
		if let Some(entry) = blackboard.storage.get(key) {
			return Some(Arc::clone(entry));
		}
		// Couldn't find key. Try remapping if we have a parent
		else if let Some(parent_bb) = self.parent.as_mut() {
			if let Some(new_key) = blackboard.internal_to_external.get(key) {
				// Return the value of the parent's `get()`
				let parent_entry = parent_bb.get_entry(new_key);

				if let Some(value) = &parent_entry {
					blackboard
						.storage
						.insert(key.to_string(), Arc::clone(value));
				}

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

	fn create_entry<'a>(&'a mut self, key: &'a (impl AsRef<str> + Sync)) -> EntryPtr {
		let entry;

		let mut blackboard = self.data.write();

		// If the entry already exists
		if let Some(existing_entry) = blackboard.storage.get(key.as_ref()) {
			return Arc::clone(existing_entry);
		}
		// Use explicit remapping rule
		else if blackboard
			.internal_to_external
			.contains_key(key.as_ref())
			&& self.parent.is_some()
		{
			// Safe to unwrap because .contains_key() is true
			let remapped_key = blackboard
				.internal_to_external
				.get(key.as_ref())
				.unwrap_or_else(|| todo!());

			entry = (*self.parent)
				.as_mut()
				.unwrap_or_else(|| todo!())
				.create_entry(remapped_key);
		}
		// Use autoremapping
		else if blackboard.auto_remapping && self.parent.is_some() {
			entry = (*self.parent)
				.as_mut()
				.unwrap_or_else(|| todo!())
				.create_entry(key);
		}
		// No remapping or no parent blackboard
		else {
			// Create an entry with an empty placeholder value
			entry = Arc::new(Mutex::new(Entry::Generic(Box::new(()))));
		}

		blackboard
			.storage
			.insert(key.as_ref().to_string(), Arc::clone(&entry));
		entry
	}

	/// Internal method that just tries to get value at key. If the stored
	/// type is not T, return None
	fn __get_no_string<T>(&mut self, key: &str) -> Option<T>
	where
		T: Any + Clone,
	{
		self.get_entry(key).and_then(|entry| {
			let entry = entry.lock();

			match &*entry {
				Entry::Generic(entry) => {
					// Try to downcast directly to T
					entry.downcast_ref::<T>().cloned()
				}
				// Because `Stringy` is a superset of `Generic`, we can return a `Stringy`
				// entry from this
				Entry::Stringy(entry) => {
					// Try to downcast directly to T
					<dyn Any>::downcast_ref::<T>(entry).cloned()
				}
			}
		})
	}

	/// Internal method that tries to get the value at key, but only works
	/// if it's a String/&str, then tries [`FromString`] to convert it to T.
	/// Treats the [`Entry`] as [`Entry::Generic`]
	fn __get_allow_string<T>(&mut self, key: &str) -> Option<T>
	where
		T: Any + Clone + FromString + Send,
	{
		// Try to get the key
		if let Some(entry) = self.get_entry(key) {
			let value = self.__get_string(key)?;

			// Try to parse String into T
			if let Ok(value) = <String as ParseStr<T>>::parse_str(&value) {
				// Update value with the value type instead of just a string
				// Because this is the non-`stringy` function, we have to update it as a
				// `Generic`
				*entry.lock() = Entry::Generic(Box::new(value.clone()));
				return Some(value);
			}
		}

		// No matches
		None
	}

	/// Internal method that tries to get the value at key as a
	/// `String` or `&str`, returning an owned type
	fn __get_string(&mut self, key: &str) -> Option<String> {
		self.get_entry(key).and_then(|entry| {
			let entry_lock = entry.lock();
			// If value is a String or &str, try to call `FromString` to convert to T
			match &(*entry_lock) {
				Entry::Generic(entry) => entry
					.downcast_ref::<String>()
					.map(ToString::to_string)
					.or_else(|| {
						entry
							.downcast_ref::<&str>()
							.map(ToString::to_string)
					}),
				Entry::Stringy(entry) => <dyn Any>::downcast_ref::<String>(entry)
					.map(ToString::to_string)
					.or_else(|| <dyn Any>::downcast_ref::<String>(entry).cloned()),
			}
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
pub enum Entry {
	/// @TODO:
	Generic(Box<dyn Any + Send>),
	/// @TODO:
	Stringy(Box<dyn AnyStringy>),
}
// endregion:   --- Entry
