// Copyright © 2025 Stephan Kunz

//! Factory for creation and modification of [`BehaviorTree`]s
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::string::{String, ToString};
use roxmltree::Document;

use crate::{
	factory::xml_parser::XmlParser,
	new_behavior::{
		BehaviorAllMethods, ComplexBhvrTickFn, NewBehaviorType, SimpleBehavior, SimpleBhvrTickFn,
		action::Script,
		condition::script_condition::ScriptCondition,
		control::{
			fallback::Fallback, parallel::Parallel, parallel_all::ParallelAll,
			reactive_fallback::ReactiveFallback, reactive_sequence::ReactiveSequence,
			sequence::Sequence, sequence_with_memory::SequenceWithMemory,
		},
		decorator::{inverter::Inverter, retry_until_successful::RetryUntilSuccessful},
	},
	new_blackboard::NewBlackboard,
	new_port::NewPortList,
	tree::{BehaviorTree, BehaviorTreeComponentContainer},
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
#[derive(Default)]
pub struct NewBehaviorTreeFactory {
	blackboard: NewBlackboard,
	registry: BehaviorRegistry,
	main_tree_name: String,
	main_tree_xml: String,
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
		// core controls
		self.register_node_type::<Fallback>("Fallback")?;
		self.register_node_type::<Parallel>("Parallel")?;
		self.register_node_type::<ParallelAll>("ParallelAll")?;
		self.register_node_type::<ReactiveFallback>("ReactiveFallback")?;
		self.register_node_type::<ReactiveSequence>("ReactiveSequence")?;
		self.register_node_type::<Sequence>("Sequence")?;
		self.register_node_type::<SequenceWithMemory>("SequenceWithMemory")?;

		// core conditions
		self.register_node_type::<ScriptCondition>("ScriptCondition")?;

		// core decorators
		self.register_node_type::<Inverter>("Inverter")?;
		self.register_node_type::<RetryUntilSuccessful>("RetryUntilSuccessful")?;

		// core actions
		self.register_node_type::<Script>("Script")
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
		if root.attribute("main_tree_to_execute").is_some() {
			self.main_tree_name = root
				.attribute("main_tree_to_execute")
				.unwrap_or("MainTree")
				.to_string();
			XmlParser::parse_root_element(
				&self.blackboard,
				&mut self.registry,
				&mut tree,
				root,
				&self.main_tree_name,
				true,
				&mut self.main_tree_xml,
			)?;
		} else {
			return Err(Error::NoTreeToExecute);
		}

		Ok(tree)
	}

	/// Create a [`BehaviorTree`] from registration
	/// # Errors
	/// - if behaviors are missing
	pub fn create_tree(&mut self) -> Result<BehaviorTree, Error> {
		extern crate std;
		let input = self.main_tree_xml.clone();
		std::dbg!(&input);
		self.main_tree_xml = String::new();
		let doc = Document::parse(&input)?;
		let root = doc.root_element();
		let mut tree = BehaviorTree::default();

		XmlParser::build_subtree(
			&self.blackboard,
			&mut self.registry,
			&mut tree,
			&self.main_tree_name,
			root,
			&self.main_tree_xml,
			false,
		)?;

		Ok(tree)
	}

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		self.registry.list_behaviors();
	}

	/// @TODO:
	/// # Errors
	pub fn register_behavior_tree_from_text(&mut self, xml: &str) -> Result<(), Error> {
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
		self.main_tree_name = root
			.attribute("main_tree_to_execute")
			.unwrap_or("MainTree")
			.to_string();
		let mut tree = BehaviorTree::default();
		XmlParser::parse_root_element(
			&self.blackboard,
			&mut self.registry,
			&mut tree,
			root,
			&self.main_tree_name,
			false,
			&mut self.main_tree_xml,
		)
	}

	/// Register a behavior plugin.
	/// # Errors
	#[allow(unsafe_code)]
	pub fn register_from_plugin(&mut self, name: impl Into<String>) -> Result<(), Error> {
		let name = name.into();
		// @TODO: handle multiplattform and multipath
		// for now the path is hardcoded
		// /home/stephan/dbx/dimas-fw/dimas/target/debug/libtest_behaviors.so
		//let libname = String::from("./") + name + ".so";
		let libname = if name == "libtest_behaviors" {
			"/home/stephan/dbx/dimas-fw/dimas/target/debug/libtest_behaviors.so"
		} else if name == "libcross_door" {
			"/home/stephan/dbx/dimas-fw/dimas/target/debug/libcross_door.so"
		} else {
			""
		};

		let lib = unsafe {
			let lib = libloading::Library::new(libname)?;
			let registration_fn: libloading::Symbol<unsafe extern "Rust" fn(&mut Self) -> u32> =
				lib.get(b"register")?;
			let res = registration_fn(&mut *self);
			if res != 0 {
				return Err(Error::RegisterLib(name, res));
			}
			lib
		};

		// The Library must be kept in storage until the [`BehaviorTree`] is destroyed.
		// Therefore the library is handed over to the behavior registry, which is later owned by tree.
		self.registry.add_library(lib);
		Ok(())
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

	/// Heelper function to print a tree recursively
	pub fn print_tree_recursively(_root_node: &BehaviorTreeComponentContainer) {
		todo!()
	}
}
// endregion:   --- BehaviorTreeFactory
