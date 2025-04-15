// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]

//! [`BehaviorRegistry`] library
//!

// region:      --- modules
use alloc::{borrow::ToOwned, boxed::Box, string::String, sync::Arc};
use hashbrown::HashMap;

use crate::new_behavior::{BehaviorCreationFn, BehaviorMethods, NewBehaviorType};

use super::error::Error;
// endregion:   --- modules

// region:     --- BehaviorRegistry
/// A registry for [`Behavior`]s used by the [`BehaviorTreeFactory`] for creation of [`BehaviorTree`]s
#[derive(Default)]
pub struct BehaviorRegistry {
	behaviors: HashMap<String, (NewBehaviorType, Arc<BehaviorCreationFn>)>,
}

impl BehaviorRegistry {
	pub fn insert<F>(
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

	pub fn find(&self, id: &str) -> Result<(NewBehaviorType, Arc<BehaviorCreationFn>), Error> {
		let (bhvr_type, creation_fn) = self
			.behaviors
			.get(id)
			.ok_or_else(|| Error::BehaviorNotRegistered(id.into()))?;
		Ok((bhvr_type.to_owned(), creation_fn.clone()))
	}
}
// endregion:   --- BehaviorRegistry
