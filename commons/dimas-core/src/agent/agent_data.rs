// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::string::String;

use crate::{ComponentData, Operational, OperationalData};

/// `AgentData`
#[derive(Debug, Default)]
pub struct AgentData {
	/// [`Operational`] data
	pub operational: OperationalData,
	/// [`ComponentData`] data
	pub component: ComponentData,
}
