// Copyright © 2025 Stephan Kunz

//! Factory for creation and modification of [`BehaviorTree`]s
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::string::String;
use roxmltree::Document;

use crate::{
	factory::xml_parser::XmlParser,
	new_behavior::{
		BehaviorAllMethods, ComplexBhvrTickFn, NewBehaviorType, SimpleBehavior, SimpleBhvrTickFn,
		action::Script,
		condition::script_condition::ScriptCondition,
		control::{
			fallback::Fallback, reactive_fallback::ReactiveFallback,
			reactive_sequence::ReactiveSequence, sequence::Sequence,
			sequence_with_memory::SequenceWithMemory,
		},
	},
	new_blackboard::NewBlackboard,
	new_port::NewPortList,
	tree::BehaviorTree,
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
#[derive(Default)]
pub struct NewBehaviorTreeFactory {
	blackboard: NewBlackboard,
	registry: BehaviorRegistry,
}

impl NewBehaviorTreeFactory {
	/// Create a factory with registered core behaviors
	/// # Errors
	/// - if core behaviors cannot be registered
	pub fn with_core_behaviors() -> Result<Self, Error> {
		let mut factory = Self::default();
		factory.core_behaviors()?;
		Ok(factory)
	}

	/// register core behaviors
	/// # Errors
	/// - if any registration fails
	pub fn core_behaviors(&mut self) -> Result<(), Error> {
		self.register_node_type::<Fallback>("Fallback")?;
		self.register_node_type::<ReactiveFallback>("ReactiveFallback")?;
		self.register_node_type::<ReactiveSequence>("ReactiveSequence")?;
		self.register_node_type::<Sequence>("Sequence")?;
		self.register_node_type::<SequenceWithMemory>("SequenceWithMemory")?;

		self.register_node_type::<Script>("Script")?;
		self.register_node_type::<ScriptCondition>("ScriptCondition")
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
			XmlParser::parse_root_element(
				&self.blackboard,
				&mut self.registry,
				&mut tree,
				root,
				true,
			)?;
		} else {
			return Err(Error::NoTreeToExecute);
		}

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
	pub fn register_from_plugin(&mut self, name: impl Into<String>) -> Result<(), Error> {
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
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_node_type<T>(&mut self, name: impl Into<String>) -> Result<(), Error>
	where
		T: BehaviorAllMethods,
	{
		let bhvr_creation_fn = T::create();
		let bhvr_type = T::kind();
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`Action`].
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_simple_action(
		&mut self,
		name: impl Into<String>,
		tick_fn: SimpleBhvrTickFn,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Action;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`Action`].
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_simple_action_with_ports(
		&mut self,
		name: impl Into<String>,
		tick_fn: ComplexBhvrTickFn,
		port_list: NewPortList,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create_with_ports(tick_fn, port_list);
		let bhvr_type = NewBehaviorType::Action;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`Condition`].
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_simple_condition(
		&mut self,
		name: impl Into<String>,
		tick_fn: SimpleBhvrTickFn,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Condition;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`Decorator`].
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	pub fn register_simple_decorator(
		&mut self,
		name: impl Into<String>,
		tick_fn: SimpleBhvrTickFn,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = NewBehaviorType::Decorator;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}
}
// endregion:   --- BehaviorTreeFactory
