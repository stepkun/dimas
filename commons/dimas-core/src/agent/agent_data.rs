// Copyright © 2024 Stephan Kunz
#![allow(unused)]

#[doc(hidden)]
extern crate alloc;

use alloc::string::String;
use uuid::Uuid;

use crate::{operational::{self, OperationalData}, ComponentData};

/// `AgentData`
#[derive(Debug, Default)]
pub struct AgentData {
	/// domain prefix
	pub prefix: String,
	/// [`Operational`] data
	pub operational: OperationalData,
	/// [`ComponentData`] data
	pub component: ComponentData,
}
