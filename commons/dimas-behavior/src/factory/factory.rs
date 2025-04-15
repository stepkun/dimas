// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]
#![allow(unused)]

//! Factory for creation and modification of [`BehaviorTree`]s
//!

use alloc::boxed::Box;
use hashbrown::HashMap;
// region:      --- modules
use roxmltree::Document;

use crate::{
	blackboard::Blackboard,
	factory::xml_parser::XmlParser,
	new_behavior::{
		BehaviorCreation, BehaviorMethods, BehaviorResult, BhvrTickFn, NewBehaviorType,
		SimpleBehavior, control::sequence::Sequence,
	},
	tree::BehaviorTree,
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
pub struct NewBehaviorTreeFactory {
	blackboard: Blackboard,
	registry: BehaviorRegistry,
}

impl core::default::Default for NewBehaviorTreeFactory {
	fn default() -> Self {
		let mut registry = BehaviorRegistry::default();
		// register core behaviors
		let bhvr_creation_fn = Sequence::create();
		registry.insert("Sequence", bhvr_creation_fn, NewBehaviorType::Control);
		Self {
			blackboard: Blackboard::default(),
			registry,
		}
	}
}

impl NewBehaviorTreeFactory {
	/// Create a [`BehaviorTree`] from XML
	/// # Errors
	/// - if XML is not well formatted
	pub fn create_from_text(&mut self, xml: &str) -> Result<BehaviorTree, Error> {
		// general checks
		let doc = Document::parse(xml)?;
		let root = doc.root_element();
		if root.tag_name().name() != "root" {
			return Err(Error::WrongRootName);
		}
		if let Some(format) = root.attribute("BTCPP_format") {
			if format != "4" {
				return Err(Error::BtCppFormat);
			}
		}
		let mut tree = BehaviorTree::default();
		if let Some(id) = root.attribute("main_tree_to_execute") {
			tree.set_root_id(id);
		} else {
			return Err(Error::NoTreeToExecute);
		};

		XmlParser::parse_root_element(&self.blackboard, &mut self.registry, &mut tree, root)?;

		Ok(tree)
	}

	/// Register a behavior plugin.
	pub fn register_from_plugin(&mut self) {
		todo!()
	}

	/// Register a [`Behavior`] of type <T>.
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_node_type<T>(&mut self, name: &str)
	where
		T: BehaviorMethods + BehaviorCreation,
	{
		let bhvr_creation_fn = T::create();
		let bhvr_type = T::kind();
		self.registry
			.insert(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Action`].
	pub fn register_simple_action(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Action;
		self.registry
			.insert(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Condition`].
	pub fn register_simple_condition(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Condition;
		self.registry
			.insert(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Decorator`].
	pub fn register_simple_decorator(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Decorator;
		self.registry
			.insert(name, bhvr_creation_fn, bhvr_type);
	}
}
// endregion:   --- BehaviorTreeFactory
