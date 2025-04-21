// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `DiMAS` implementation for a subtree behavior

// region:      --- modules
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use parking_lot::Mutex;

use crate::{
	new_blackboard::NewBlackboard,
	new_port::NewPortList,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentContainer},
};

use super::{
	BehaviorAllMethods, BehaviorConfigurationData, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods, BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods, NewBehaviorStatus, NewBehaviorType
};
// endregion:   --- modules

// region:      --- SubtreeCallee
/// A `SubtreeCallee`
#[derive(Debug, Default)]
pub struct SubtreeCallee {
	id: String,
	blackboard: NewBlackboard,
	children: Vec<BehaviorTreeComponentContainer>,
	config_data: BehaviorConfigurationData,
}

impl SubtreeCallee {
	/// create a Subtree as behavior tree node.
	#[must_use]
	pub fn create(
		id: impl Into<String>,
		blackboard: NewBlackboard,
		children: Vec<BehaviorTreeComponentContainer>,
		config_data: BehaviorConfigurationData,
	) -> Arc<Mutex<Self>> {
		Arc::new(Mutex::new(Self {
			id: id.into(),
			blackboard,
			children,
			config_data,
		}))
	}

	/// Get the Blackboard
	#[must_use]
	pub const fn blackboard(&self) -> &NewBlackboard {
		&self.blackboard
	}

	/// Get the children
	#[must_use]
	pub const fn children(&self) -> &Vec<BehaviorTreeComponentContainer> {
		&self.children
	}

	/// Get the children mutable
	#[must_use]
	pub const fn children_mut(&mut self) -> &mut Vec<BehaviorTreeComponentContainer> {
		&mut self.children
	}

	/// Get the id
	#[must_use]
	pub fn id(&self) -> &str {
		&self.id
	}

	/// Tick function
	/// # Errors
	pub fn execute_tick(&mut self) -> BehaviorResult {
		self.children[0].execute_tick()
	}

	/// Halt function
	/// # Errors
	pub fn execute_halt(&mut self) -> BehaviorResult {
		self.children[0].execute_halt()
	}
}

// endregion:   --- SubtreeCallee
