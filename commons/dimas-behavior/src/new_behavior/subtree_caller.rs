// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `DiMAS` implementation for a subtree behavior

use alloc::{boxed::Box, format, string::String, sync::Arc, vec::Vec};
use parking_lot::Mutex;

// region:      --- modules
use crate::{
	new_port::NewPortList,
	tree::{BehaviorTreeComponent, BehaviorTreeComponentContainer},
};

use super::{
	BehaviorConfigurationData, BehaviorInstanceMethods, BehaviorRedirectionMethods, BehaviorResult,
	BehaviorTickData, BehaviorTreeMethods, SubtreeCallee, error::NewBehaviorError,
};
// endregion:   --- modules

// region:      --- SubtreeCaller
/// A Subtree caller
#[derive(Debug)]
pub struct SubtreeCaller {
	/// Id of the called subtree
	callee_id: String,
	/// The called subtree may not exist on creation
	callee: Option<Arc<Mutex<SubtreeCallee>>>,
}

impl BehaviorTreeMethods for SubtreeCaller {}

impl BehaviorInstanceMethods for SubtreeCaller {
	fn start(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		self.callee.as_ref().map_or_else(
			|| Err(NewBehaviorError::Composition("no behaviors in tree".into())),
			|subtree| subtree.lock().execute_tick(),
		)
	}

	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		self.callee.as_ref().map_or_else(
			|| Err(NewBehaviorError::Composition("no behaviors in tree".into())),
			|subtree| subtree.lock().execute_tick(),
		)
	}

	fn halt(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		self.callee.as_ref().map_or_else(
			|| Err(NewBehaviorError::Composition("no behaviors in tree".into())),
			|subtree| subtree.lock().execute_halt(),
		)
	}
}

impl BehaviorRedirectionMethods for SubtreeCaller {
	#[allow(clippy::option_if_let_else)]
	fn static_provided_ports(&self) -> NewPortList {
		NewPortList::default()
	}
}

impl SubtreeCaller {
	/// @TODO:
	fn bhvr_creation(callee_id: &str, callee: Option<Arc<Mutex<SubtreeCallee>>>) -> Box<dyn BehaviorTreeMethods> {
		let callee_id = callee_id.into();
		Box::new(Self { callee_id, callee })
	}

	/// @TODO:
	#[allow(clippy::needless_pass_by_value)]
	#[must_use]
	pub fn create_node(
		callee_id: &str,
		callee: Option<Arc<Mutex<SubtreeCallee>>>,
		tick_data: BehaviorTickData,
		config_data: BehaviorConfigurationData,
	) -> BehaviorTreeComponentContainer {
		let behavior = Self::bhvr_creation(callee_id, callee);
		BehaviorTreeComponentContainer::create_node(
			behavior,
			tick_data,
			Vec::new(),
			config_data,
		)
	}
}
// endregion:   --- SubtreeCaller
