// Copyright © 2024 Stephan Kunz

//! A register for components
//!

extern crate std;

// region:      --- modules
use anyhow::Result;
use dimas_core::{Component, ComponentId, ComponentRegistrar, OperationState};
use std::collections::HashMap;
use std::prelude::v1::Box;
// endregion:   --- modules

/// Library loader implementation
#[derive(Debug)]
pub struct ComponentRegister {
	/// Storage for the [`Component`]s
	pub components: HashMap<ComponentId, Box<dyn Component>>,
}

impl Default for ComponentRegister {
	/// Create a default [`ComponentRegister`]
	#[must_use]
	fn default() -> Self {
		Self::new()
	}
}

impl ComponentRegistrar for ComponentRegister {
	fn register(&mut self, plugin: Box<dyn Component>) {
		self.components.insert(plugin.id(), plugin);
	}

	fn deregister(&mut self, id: &ComponentId) -> Result<Option<Box<dyn Component>>> {
		let mut plugin = self.components.remove(id);
		let downstate = OperationState::Configured;
		// shutdown plugin
		plugin = if let Some(plugin) = plugin {
			plugin.manage_operation_state_old(downstate)?;
			Some(plugin)
		} else {
			None
		};
		Ok(plugin)
	}
}

impl ComponentRegister {
	/// Creates a [`ComponentRegister`]
	#[must_use]
	pub fn new() -> Self {
		Self {
			components: HashMap::new(),
		}
	}
}
