// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use core::fmt::Debug;
use tracing::{error, event, info, instrument, warn, Level};
use uuid::Uuid;

use crate::{Activity, Agent, ManageOperationState, OperationState};

use super::{Component, ComponentId};

/// `ComponentData`
#[derive(Default)]
pub struct ComponentData {
	/// components id
	pub id: ComponentId,
	/// unique id
	pub uuid: Uuid,
	/// components version
	pub version: u32,
	/// The context [`Agent`]
	pub ctx: Option<Agent>,
	/// list of created activities
	pub activities: Vec<Box<dyn Activity>>,
	/// list of contained sub components
	pub components: Vec<Box<dyn Component>>,
}

impl ComponentData {
	/// Creation of [`ComponentData`]
	#[must_use]
	pub fn new(id: impl Into<ComponentId>, version: u32, ctx: Agent) -> Self {
		Self {
			id: id.into(),
			uuid: Uuid::new_v4(),
			version,
			ctx: Some(ctx),
			activities: Vec::default(),
			components: Vec::default(),
		}
	}
}

/// Manual implementation due to possible infinite recursion.
/// References to other components might create a loop.
impl Debug for ComponentData {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("ComponentData")
			.field("id", &self.id)
			.field("uuid", &self.uuid)
			.field("version", &self.version)
			.field("ctx", &self.ctx.is_some())
			.field("activities", &self.activities)
			.field("components", &self.components)
			.finish_non_exhaustive()
	}
}
