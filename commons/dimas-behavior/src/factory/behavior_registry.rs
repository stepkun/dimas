// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]

//! [`BehaviorRegistry`] library
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{borrow::ToOwned, boxed::Box, string::String, sync::Arc, vec::Vec};
use hashbrown::HashMap;
use libloading::Library;

use crate::new_behavior::{BehaviorCreationFn, BehaviorMethods, NewBehaviorType};

use super::error::Error;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for [`Behavior`]s used by the [`BehaviorTreeFactory`] for creation of [`BehaviorTree`]s
#[derive(Default)]
pub struct BehaviorRegistry {
	behaviors: HashMap<String, (NewBehaviorType, Arc<BehaviorCreationFn>)>,
	librarys: Vec<Library>,
}

impl BehaviorRegistry {
	/// Add a behavior to the registry
	pub fn add_behavior<F>(
		&mut self,
		name: impl AsRef<str>,
		bhvr_creation_fn: F,
		bhvr_type: NewBehaviorType,
	) where
		F: Fn() -> Box<dyn BehaviorMethods> + Send + Sync + 'static,
	{
		self.behaviors.insert(
			name.as_ref().into(),
			(bhvr_type, Arc::new(bhvr_creation_fn)),
		);
	}

	/// The Library must be kept in storage until the [`BehaviorTree`] is destroyed.
	/// Therefore the library is stored in the behavior registry, which is later owned by tree.
	/// The `add_library(..)` function also takes care of registering all 'symbols'.
	/// # Errors
	#[allow(unsafe_code)]
	pub fn add_library(&mut self, name: &str, library: Library) -> Result<(), Error> {
		unsafe {
			let registration_fn: libloading::Symbol<unsafe extern "Rust" fn(&mut Self) -> u32> =
				library.get(b"register")?;
			let res = registration_fn(self);
			if res != 0 {
				return Err(Error::RegisterLib(name.into(), res));
			}
		}

		self.librarys.push(library);
		Ok(())
	}

	/// Register a behavior in the registry
	pub extern "Rust" fn register_behavior(
		&mut self,
		name: impl AsRef<str>,
		bhvr_creation_fn: Box<dyn Fn() -> Box<dyn BehaviorMethods> + Send + Sync + 'static>,
		bhvr_type: NewBehaviorType,
	) {
		self.behaviors.insert(
			name.as_ref().into(),
			(bhvr_type, Arc::from(bhvr_creation_fn)),
		);
	}

	/// Get a reference to the beaviors
	#[must_use]
	pub fn behaviors(&self) -> &HashMap<String, (NewBehaviorType, Arc<BehaviorCreationFn>)> {
		&self.behaviors
	}
	/// Find a behavior in the registry
	/// # Errors
	/// - if the behavior is not in the registry
	pub fn find(&self, id: &str) -> Result<(NewBehaviorType, Arc<BehaviorCreationFn>), Error> {
		// extern crate std;
		// std::println!("find {id} in ");
		// self.list_behaviors();
		let (bhvr_type, creation_fn) = self
			.behaviors
			.get(id)
			.ok_or_else(|| Error::BehaviorNotRegistered(id.into()))?;
		Ok((bhvr_type.to_owned(), creation_fn.clone()))
	}

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		for (key, _) in &self.behaviors {
			std::println!("{key}");
		}
		std::println!();
	}
}
// endregion:   --- BehaviorRegistry
