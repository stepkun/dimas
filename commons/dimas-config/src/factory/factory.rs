// Copyright © 2024 Stephan Kunz

//! [`BehaviorTree`] factory of `DiMAS`

// region:      --- modules
use alloc::{string::String, sync::Arc, vec::Vec};
use core::fmt::Debug;
use dimas_core::{
	behavior::{tree::BehaviorTree, Behavior, BehaviorCategory, BehaviorConfig},
	blackboard::Blackboard,
	build_bhvr_ptr,
};
use hashbrown::HashMap;
use roxmltree::{Document, NodeType, ParsingOptions};
use tracing::{instrument, Level};

use crate::builtin::{
	control::{Fallback, Sequence},
	decorator::{Inverter, Retry},
};

use super::{error::Error, xml_parser::XmlParser};
// endregon:    --- modules

// region:      --- types
/// @TODO:
type BehaviorCreateFn = dyn Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync;
// endregion:   --- types

// region:      --- FactoryData
#[allow(clippy::module_name_repetitions)]
pub struct FactoryData {
	pub main_tree_id: Option<String>,
	pub bhvr_map: HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)>,
	tree_roots: HashMap<String, Behavior>,
}

impl FactoryData {
	fn create_fundamentals() -> HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)> {
		let mut map: HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)> = HashMap::new();

		// Fallback
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut node = build_bhvr_ptr!(config, "Fallback", Fallback);
			node.data.children = children;
			node
		};
		map.insert(
			"Fallback".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// Inverter
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut node = build_bhvr_ptr!(config, "Inverter", Inverter);
			node.data.children = children;
			node
		};
		map.insert(
			"Inverter".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// Retry
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut node = build_bhvr_ptr!(config, "Retry", Retry);
			node.data.children = children;
			node
		};
		map.insert(
			"Retry".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// Sequence
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut node = build_bhvr_ptr!(config, "Sequence", Sequence);
			node.data.children = children;
			node
		};
		map.insert(
			"Sequence".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		map
	}
}

impl Default for FactoryData {
	fn default() -> Self {
		Self {
			main_tree_id: None,
			bhvr_map: Self::create_fundamentals(),
			tree_roots: HashMap::new(),
		}
	}
}
// endregion:      --- FactoryData

// region:      --- BTFactory
/// @TODO:
#[allow(clippy::module_name_repetitions)]
pub struct BTFactory {
	root_blackboard: Blackboard,
	data: FactoryData,
}

impl BTFactory {
	/// Create an empty behavior factory using the given [`Blackboard`].
	#[must_use]
	pub fn new(blackboard: Blackboard) -> Self {
		Self {
			root_blackboard: blackboard,
			data: FactoryData::default(),
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn blackboard(&self) -> &Blackboard {
		&self.root_blackboard
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn create_tree(&mut self, xml: &str) -> Result<BehaviorTree, Error> {
		// remove leading linebreaks, as those lead to an error
		let xml = xml.trim_start_matches('\n');
		let parser = XmlParser::default();

		let root_node = parser.parse_root(&self.root_blackboard, &mut self.data, xml)?;
		Ok(BehaviorTree::new(root_node))
	}

	/// @TODO:
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn register_behavior<F>(
		&mut self,
		name: impl Into<String>,
		bhvr_fn: F,
		bhvr_type: BehaviorCategory,
	) where
		F: Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync + 'static,
	{
		self.data
			.bhvr_map
			.insert(name.into(), (bhvr_type, Arc::new(bhvr_fn)));
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn register_subtree(&mut self, xml: &str) -> Result<(), Error> {
		// remove leading linebreaks, as those lead to an error
		let xml = xml.trim_start_matches('\n');
		let parser = XmlParser::default();

		parser.parse_subtree(&self.root_blackboard, &mut self.data, xml)
	}
}

impl Debug for BTFactory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("BTFactory")
			.field("blackboard", &self.root_blackboard)
			//.field("bhvr_map", &self.bhvr_map)
			.finish_non_exhaustive()
	}
}

impl Default for BTFactory {
	fn default() -> Self {
		Self::new(Blackboard::default())
	}
}
// endregion:   --- BTFactory
