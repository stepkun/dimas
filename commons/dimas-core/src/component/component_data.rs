// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::string::String;
use uuid::Uuid;

use super::ComponentId;

/// `ComponentData`
#[derive(Debug)]
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
