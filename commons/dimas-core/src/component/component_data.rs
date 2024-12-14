// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::string::String;
use core::fmt::Debug;
use uuid::Uuid;

use crate::Agent;

use super::ComponentId;

/// `ComponentData`
#[derive(Default)]
pub struct ComponentData {
	/// unique id
	pub uuid: Uuid,
	/// components version
	pub version: u32,
	/// components id
	pub id: ComponentId,
}

impl ComponentData {
	/// Creation of [`ComponentData`]
	#[must_use]
	pub fn new(uuid: Uuid, id: impl Into<ComponentId>, version: u32) -> Self {
		Self {
			uuid,
			version,
			id: id.into(),
		}
	}
}

/// Manual implementation due to possible infinite recursion
/// because of optional reference to parent [`Agent`].
impl Debug for ComponentData {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("ComponentData")
			.field("uuid", &self.uuid)
			.field("version", &self.version)
			.field("id", &self.id)
			.finish_non_exhaustive()
	}
}
