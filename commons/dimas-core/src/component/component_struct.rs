// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::derivable_impls)]

#[doc(hidden)]
extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use uuid::Uuid;

use crate::{Activity, Component};

/// `ComponentStruct`
#[derive(Debug)]
pub struct ComponentStruct {
	/// list of created activities
	pub activities: Vec<Box<dyn Activity>>,
	/// list of contained sub components
	pub components: Vec<Box<dyn Component>>,
}

impl Default for ComponentStruct {
	fn default() -> Self {
		Self {
			activities: Vec::default(),
			components: Vec::default(),
		}
	}
}
