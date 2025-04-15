// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]
#![allow(unused)]

//! Factory for creation and modification of [`BehaviorTree`]s
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{boxed::Box, string::String};
use hashbrown::HashMap;
use roxmltree::Document;

use crate::{
	blackboard::Blackboard,
	factory::xml_parser::XmlParser,
	new_behavior::{
		BehaviorCreation, BehaviorMethods, BehaviorResult, BhvrTickFn, NewBehaviorType,
		SimpleBehavior,
		control::{
			reactive_sequence::ReactiveSequence, sequence::Sequence,
			sequence_with_memory::SequenceWithMemory,
		},
	},
	tree::BehaviorTree,
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
#[derive(Default)]
pub struct NewBehaviorTreeFactory {
	blackboard: Blackboard,
	registry: BehaviorRegistry,
}

impl NewBehaviorTreeFactory {
	/// Creat a factory with registered core behaviors
	#[must_use]
	pub fn with_core_behaviors() -> Self {
		let mut factory = Self::default();
		factory.core_behaviors();
		factory
	}

	/// register core behaviors
	pub fn core_behaviors(&mut self) {
		self.register_node_type::<ReactiveSequence>("ReactiveSequence");
		self.register_node_type::<Sequence>("Sequence");
		self.register_node_type::<SequenceWithMemory>("SequenceWithMemory");
	}

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

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		self.registry.list_behaviors();
	}

	/// Register a behavior plugin.
	/// # Errors
	#[allow(unsafe_code)]
	pub fn register_from_plugin(&mut self, name: &str) -> Result<(), Error> {
		// @TODO: handle multiplattform and multipath
		// for now the path is hardcoded
		// /home/stephan/dbx/dimas-fw/dimas/target/debug/libtest_behaviors.so
		//let libname = String::from("./") + name + ".so";
		let libname = "/home/stephan/dbx/dimas-fw/dimas/target/debug/libtest_behaviors.so";
		let lib = unsafe { libloading::Library::new(libname)? };

		// The Library must be kept in storage until the [`BehaviorTree`] is destroyed.
		// Therefore the library is handed over the behavior registry, which is later owned by tree.
		self.registry.add_library(name, lib)
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
			.add_behavior(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Action`].
	pub fn register_simple_action(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Action;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Condition`].
	pub fn register_simple_condition(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Condition;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type);
	}

	/// Register a function as [`Decorator`].
	pub fn register_simple_decorator(&mut self, name: &str, tick_fn: BhvrTickFn) {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Decorator;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type);
	}
}
// endregion:   --- BehaviorTreeFactory
