// Copyright © 2024 Stephan Kunz

//! A register for components
//!

extern crate std;

// region:      --- modules
use anyhow::Result;
use dimas_core::{Component, ComponentId, OperationState};
use std::collections::HashMap;

use super::ComponentRegistry;
// endregion:   --- modules

/// Library loader implementation
#[derive(Debug)]
pub struct ComponentRegistryType {
	/// Storage for the [`Component`]s
	pub components: HashMap<ComponentId, Box<dyn Component>>,
}

impl Default for ComponentRegistryType {
	/// Create a default [`ComponentRegister`]
	#[must_use]
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentRegistry for ComponentRegistryType {
	fn register(&mut self, plugin: Box<dyn Component>) {
		self.components.insert(plugin.id(), plugin);
	}

	fn deregister(&mut self, id: &ComponentId) -> Result<Option<Box<dyn Component>>> {
		let mut plugin = self.components.remove(id);
		let downstate = OperationState::Configured;
		// shutdown plugin
		plugin = if let Some(mut plugin) = plugin {
			plugin.manage_operation_state(downstate)?;
			Some(plugin)
		} else {
			None
		};
		Ok(plugin)
	}
}

impl ComponentRegistryType {
	/// Creates a [`ComponentRegister`]
	#[must_use]
	pub fn new() -> Self {
		Self {
			components: HashMap::new(),
		}
	}
}
