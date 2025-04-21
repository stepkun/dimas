// Copyright © 2025 Stephan Kunz

//! Factory for creation and modification of [`BehaviorTree`]s
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use parking_lot::Mutex;
use roxmltree::Document;

use crate::{
	factory::xml_parser::XmlParser,
	new_behavior::{
		BehaviorAllMethods, ComplexBhvrTickFn, NewBehaviorType, SimpleBehavior, SimpleBhvrTickFn,
		SubtreeCallee,
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
	main_tree_name: String,
	main_tree: Option<BehaviorTree>,
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
		// handle the attribute 'main_tree_to_execute with a default "MainTree" 
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
		)?;

		Ok(tree)
	}

	/// Create a [`BehaviorTree`] with id `MainTree` from registration
	/// # Errors
	/// - if behaviors are missing
	pub fn create_main_tree(&mut self) -> Result<BehaviorTree, Error> {
		self.create_tree("MainTree")
	}

	/// Create the named [`BehaviorTree`] from registration
	/// # Errors
	/// - if behaviors are missing
	/// # Panics
	pub fn create_tree(&mut self, name: &str) -> Result<BehaviorTree, Error> {
		if self.main_tree.is_some() {
			let tree = self.main_tree.take().expect("missing tree");
			tree.link_subtrees()?;
			Ok(tree)
		} else {
			Err(Error::NoMainTree(name.into()))
		}
	}

	/// Prints out the list of registered behaviors
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		self.registry.list_behaviors();
	}

	/// @TODO:
	/// # Errors
	/// # Panics
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
		// handle the attribute 'main_tree_to_execute with a default "MainTree" 
		self.main_tree_name = root
			.attribute("main_tree_to_execute")
			.unwrap_or("MainTree")
			.to_string();
		// on first run there is no tree stored
		let mut tree = if self.main_tree.is_some() {
			self.main_tree.take().expect("build error: tree is missing!")
		} else {
			BehaviorTree::default()
		};
		XmlParser::register_root_element(&self.blackboard, &mut self.registry, &mut tree, root)?;
		self.main_tree = Some(tree);
		Ok(())
	}

	/// Get the name list of registered (sub)trees
	#[must_use]
	pub fn registered_behavior_trees(&self) -> Vec<String> {
		let mut res = Vec::new();
		if let Some(tree) = &self.main_tree {
			for subtree in tree.subtrees() {
				res.push(subtree.lock().id().to_string());
			}
		}
		res
	}

	/// Register a behavior plugin.
	/// For now ot is  recommended, that
	/// - the plugin resides in the executables directory and
	/// - is compiled with the same tust version.
	/// # Errors
	/// - if library is not found ore does not found
	/// - if library does not provide the `extern "Rust" register(&mut BehaviorTreeFactory) -> i32` function
	/// # Panics
	/// - on OS other than `Windows` and `Linux`,
	/// - should not panic on supported OS unless some weird constellation is happening.
	#[cfg(feature = "std")]
	#[allow(unsafe_code)]
	pub fn register_from_plugin(&mut self, name: &str) -> Result<(), Error> {
		// create path from exe path
		// in dev environment maybe we have to remove a '/deps'
		let exe_path = std::env::current_exe()?
			.parent()
			.expect("snh")
			.to_str()
			.expect("snh")
			.trim_end_matches("/deps")
			.to_string();

		#[cfg(not(any(target_os = "linux", target_os = "windows")))]
		todo!("This plattform is not upported!");
		#[cfg(target_os = "linux")]
		let libname = exe_path + "/lib" + name + ".so";
		#[cfg(target_os = "windows")]
		let libname = exe_path + "\\" + name + ".dll";

		let lib = unsafe {
			let lib = libloading::Library::new(libname)?;
			let registration_fn: libloading::Symbol<unsafe extern "Rust" fn(&mut Self) -> u32> =
				lib.get(b"register")?;
			let res = registration_fn(&mut *self);
			if res != 0 {
				return Err(Error::RegisterLib(name.into(), res));
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
		let bhvr_creation_fn = T::creation_fn();
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

	/// Print the tree structure
	pub fn print_tree(&self) {
		if let Some(tree) = &self.main_tree {
			Self::print_tree_recursively(tree.root_node());
		}
	}

	/// Helper function to print a (sub)tree recursively
	#[cfg(feature = "std")]
	pub fn print_tree_recursively(root_node: Arc<Mutex<SubtreeCallee>>) {

		Self::print_recursively(0, root_node);
	}

	/// Recursion function to print a (sub)tree recursively
	/// Limit is a tree-depth of 127
	#[cfg(feature = "std")]
	#[allow(clippy::needless_pass_by_value)]
	pub fn print_recursively(level: i8, root_node: Arc<Mutex<SubtreeCallee>>) {
		if level == i8::MAX {
			return
		}
		std::println!("- {}", root_node.lock().id());
		let next_level = level + 1;
		let mut indentation = String::new();
		for _ in 0..next_level {
			indentation.push_str("   |");
		}
		for child in root_node.as_ref().lock().children() {
			std::println!("{}- {}", indentation, child.config_data.name());
			// @TODO: Self::print_recursively(next_level, child.);
		}
	}
}
// endregion:   --- BehaviorTreeFactory
