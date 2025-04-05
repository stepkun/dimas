// Copyright © 2024 Stephan Kunz

//! [`BehaviorTree`] factory of `DiMAS`

extern crate std;

// region:      --- modules
use alloc::{string::String, sync::Arc, vec::Vec};
use core::fmt::Debug;
use dimas_behavior::{
	behavior::{Behavior, BehaviorCategory, BehaviorConfig, tree::BehaviorTree},
	blackboard::Blackboard,
	build_bhvr_ptr,
};
use hashbrown::HashMap;
use roxmltree::Document;
use tracing::{Level, instrument};

use crate::builtin::{
	action::script::Script,
	condition::script_condition::ScriptCondition,
	control::{
		Fallback, IfThenElse, Parallel, ParallelAll, ReactiveFallback, ReactiveSequence, Sequence,
		SequenceWithMemory, WhileDoElse,
	},
	decorator::{
		ForceFailure, ForceSuccess, Inverter, KeepRunningUntilFailure, Repeat, Retry,
		RetryUntilSuccessful, RunOnce,
	},
};

use super::{error::Error, xml_parser::XmlParser};
// endregon:    --- modules

// region:      --- types
/// @TODO:
pub type BehaviorCreateFn = dyn Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync;
// endregion:   --- types

// region:      --- FactoryData
#[allow(clippy::module_name_repetitions)]
pub struct FactoryData {
	pub main_tree_id: Option<String>,
	pub bhvr_map: HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)>,
	pub tree_definitions: HashMap<String, String>,
}

impl FactoryData {
	#[allow(clippy::too_many_lines)]
	fn add_extensions(&mut self) {
		// ForceFailure
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ForceFailure", ForceFailure);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ForceFailure".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// ForceSuccess
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ForceSuccess", ForceSuccess);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ForceSuccess".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// IfThenElse
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "IfThenElse", IfThenElse);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"IfThenElse".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// Inverter
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Inverter", Inverter);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"Inverter".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// KeepRunningUntilFailure
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr =
				build_bhvr_ptr!(config, "KeepRunningUntilFailure", KeepRunningUntilFailure);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"KeepRunningUntilFailure".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// ParallelAll
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ParallelAll", ParallelAll);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ParallelAll".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// ReactiveFallback
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ReactiveFallback", ReactiveFallback);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ReactiveFallback".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// ReactiveSequence
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ReactiveSequence", ReactiveSequence);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ReactiveSequence".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// Repeat
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Repeat", Repeat);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"Repeat".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// Retry
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Retry", Retry);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"Retry".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// RetryUntilSuccessful
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "RetryUntilSuccessful", RetryUntilSuccessful);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"RetryUntilSuccessful".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// RunOnce
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "RunOnce", RunOnce);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"RunOnce".into(),
			(BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
		);

		// Script
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Script", Script);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"Script".into(),
			(BehaviorCategory::Action, Arc::new(bhvr_fn)),
		);

		// ScriptCondition
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "ScriptCondition", ScriptCondition);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"ScriptCondition".into(),
			(BehaviorCategory::Condition, Arc::new(bhvr_fn)),
		);

		// SequenceWithMemory
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "SequenceWithMemory", SequenceWithMemory);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"SequenceWithMemory".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// WhileDoElse
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "WhileDoElse", WhileDoElse);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		self.bhvr_map.insert(
			"WhileDoElse".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);
	}

	fn create_fundamentals() -> HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)> {
		let mut map: HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)> =
			HashMap::default();

		// Fallback
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Fallback", Fallback);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		map.insert(
			"Fallback".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// Parallel
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Parallel", Parallel);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		map.insert(
			"Parallel".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		// Sequence
		let bhvr_fn = move |config: BehaviorConfig, children: Vec<Behavior>| -> Behavior {
			let mut bhvr = build_bhvr_ptr!(config, "Sequence", Sequence);
			bhvr.data_mut().set_children(children);
			bhvr
		};
		map.insert(
			"Sequence".into(),
			(BehaviorCategory::Control, Arc::new(bhvr_fn)),
		);

		map
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn register_behavior<F>(
		&mut self,
		name: impl Into<String>,
		bhvr_fn: F,
		bhvr_type: BehaviorCategory,
	) where
		F: Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync + 'static,
	{
		self.bhvr_map
			.insert(name.into(), (bhvr_type, Arc::new(bhvr_fn)));
	}
}

impl Default for FactoryData {
	fn default() -> Self {
		Self {
			main_tree_id: None,
			bhvr_map: Self::create_fundamentals(),
			tree_definitions: HashMap::default(),
		}
	}
}
// endregion:      --- FactoryData

// region:      --- BTFactory
/// @TODO:
#[allow(clippy::module_name_repetitions)]
pub struct BTFactory {
	blackboard: Blackboard,
	data: FactoryData,
}

impl BTFactory {
	/// Create an empty behavior factory using the given [`Blackboard`].
	#[must_use]
	pub fn extended() -> Self {
		let mut data = FactoryData::default();
		data.add_extensions();
		Self::new(Blackboard::default(), data)
	}

	/// Constructor
	#[must_use]
	pub const fn new(blackboard: Blackboard, data: FactoryData) -> Self {
		Self { blackboard, data }
	}

	/// Create an empty behavior factory using the given [`Blackboard`].
	#[must_use]
	pub fn with_blackboard(blackboard: Blackboard) -> Self {
		Self {
			blackboard,
			data: FactoryData::default(),
		}
	}

	/// @TODO:
	pub fn add_extensions(&mut self) {
		self.data.add_extensions();
	}

	/// @TODO:
	#[must_use]
	pub const fn blackboard(&self) -> &Blackboard {
		&self.blackboard
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn create_tree_from_xml(&mut self, xml: &str) -> Result<BehaviorTree, Error> {
		// first validate original xml
		let doc = Document::parse(xml)?;
		let root = doc.root_element();
		if root.tag_name().name() != "root" {
			return Err(Error::RootName);
		}

		if let Some(format) = root.attribute("BTCPP_format") {
			if format != "4" {
				return Err(Error::BtCppFormat);
			}
		}

		// shrink and create tree from validated xml
		let xml = Self::shrink_xml(xml);
		let root_bhvr = XmlParser::parse_main_xml(&self.blackboard, &mut self.data, &xml)?;
		Ok(BehaviorTree::new(root_bhvr))
	}

	/// @TODO:
	#[inline]
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
			.register_behavior(name.into(), bhvr_fn, bhvr_type);
	}

	/// @TODO:
	/// # Errors;
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn register_subtree(&mut self, xml: &str) -> Result<(), Error> {
		let xml = Self::shrink_xml(xml);
		XmlParser::parse_sub_xml(&self.blackboard, &mut self.data, &xml)
	}

	/// Reduce the input XML by eliminating everything
	/// that is not information but only formatting
	fn shrink_xml(input: &str) -> String {
		let mut res = String::with_capacity(input.len());
		let mut in_whitespaces = false;
		let mut in_tag = false;
		let mut in_literal = false;
		let mut in_assignment = false;
		for char in input.chars() {
			match char {
				// eliminate line breaks
				'\n' => continue,
				'<' => {
					in_tag = true;
					in_whitespaces = false;
					in_assignment = false;
				}
				'>' => {
					in_tag = false;
					in_whitespaces = false;
					in_assignment = false;
				}
				'"' => {
					in_literal = !in_literal;
					in_whitespaces = false;
					in_assignment = false;
				}
				' ' | '\t' => {
					if !in_tag {
						continue;
					}
					if !in_literal {
						if in_whitespaces || in_assignment {
							continue;
						}
						in_whitespaces = true;
					}
				}
				'=' => {
					if in_whitespaces {
						res.pop();
					}
					in_whitespaces = false;
					in_assignment = true;
				}
				_ => {
					in_whitespaces = false;
					in_assignment = false;
				}
			}
			res.push(char);
		}

		res.shrink_to_fit();
		//dbg!(&res);
		res
	}
}

impl Debug for BTFactory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("BTFactory")
			.field("blackboard", &self.blackboard)
			//.field("bhvr_map", &self.bhvr_map)
			.finish_non_exhaustive()
	}
}

impl Default for BTFactory {
	fn default() -> Self {
		Self::new(Blackboard::default(), FactoryData::default())
	}
}
// endregion:   --- BTFactory
