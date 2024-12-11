// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::string::String;
use uuid::Uuid;

/// `AgentData`
#[derive(Debug)]
pub struct AgentData {
	/// unique id
	pub uuid: Uuid,
	/// domain prefix
	pub prefix: String,
	/// agents name
	pub name: String,
	/// agents version
	pub version: u32,
}

impl Default for AgentData {
	fn default() -> Self {
		Self {
			uuid: Uuid::new_v4(),
			prefix: String::default(),
			name: String::default(),
			version: 0,
		}
	}
}
