// Copyright © 2025 Stephan Kunz

//! Factory for creation and modification of [`BehaviorTree`]s.
//!
//! The factory ensures that a tree is properly created and libraries or plugins
//! are loaded properly and kept in memory as long as needed.

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{string::ToString, vec::Vec};
use dimas_core::ConstString;
use roxmltree::Document;

use crate::{
	behavior::{
		Behavior, BehaviorState, BehaviorStatic, BehaviorType, ComplexBhvrTickFn, SimpleBehavior, SimpleBhvrTickFn,
		action::{Script, StateAfter},
		condition::script_condition::ScriptCondition,
		control::{
			fallback::Fallback, parallel::Parallel, parallel_all::ParallelAll, reactive_fallback::ReactiveFallback,
			reactive_sequence::ReactiveSequence, sequence::Sequence, sequence_with_memory::SequenceWithMemory,
			while_do_else::WhileDoElse,
		},
		decorator::{
			force_failure::ForceFailure, inverter::Inverter, retry_until_successful::RetryUntilSuccessful,
			script_precondition::Precondition, subtree::Subtree,
		},
	},
	factory::xml_parser::XmlParser,
	port::PortList,
	register_node,
	tree::BehaviorTree,
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
pub struct BehaviorTreeFactory {
	registry: BehaviorRegistry,
	main_tree_name: Option<ConstString>,
}

impl Default for BehaviorTreeFactory {
	fn default() -> Self {
		let mut f = Self {
			registry: BehaviorRegistry::default(),
			main_tree_name: None,
		};
		// minimum required behaviors for the factory to work
		f.register_node_type::<Subtree>("SubTree")
			.expect("snh");

		f
	}
}

impl BehaviorTreeFactory {
	/// Access the registry
	pub const fn registry(&mut self) -> &mut BehaviorRegistry {
		&mut self.registry
	}

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
		// core actions
		self.register_node_type::<Script>("Script")?;
		register_node!(self, StateAfter, "AlwaysFailure", BehaviorState::Failure, 0)?;
		register_node!(self, StateAfter, "AlwaysRunning", BehaviorState::Running, 0)?;
		register_node!(self, StateAfter, "AlwaysSuccess", BehaviorState::Success, 0)?;

		// core conditions
		self.register_node_type::<ScriptCondition>("ScriptCondition")?;

		// core controls
		self.register_node_type::<Fallback>("Fallback")?;
		self.register_node_type::<Parallel>("Parallel")?;
		self.register_node_type::<ParallelAll>("ParallelAll")?;
		self.register_node_type::<ReactiveFallback>("ReactiveFallback")?;
		self.register_node_type::<ReactiveSequence>("ReactiveSequence")?;
		self.register_node_type::<Sequence>("Sequence")?;
		self.register_node_type::<SequenceWithMemory>("SequenceWithMemory")?;
		self.register_node_type::<WhileDoElse>("WhileDoElse")?;

		// core decorators
		self.register_node_type::<ForceFailure>("ForceFailure")?;
		self.register_node_type::<Inverter>("Inverter")?;
		self.register_node_type::<RetryUntilSuccessful>("RetryUntilSuccessful")?;
		self.register_node_type::<Precondition>("Precondition")
	}

	/// Register an enums key/value pair.
	/// # Errors
	/// - if the key is already used
	pub fn register_enum_tuple(&mut self, key: &str, value: i8) -> Result<(), Error> {
		self.registry.register_enum_tuple(key, value)
	}

	/// Create a [`BehaviorTree`] directly from XML.
	/// # Errors
	/// - if XML is not well formatted
	/// - if no main tree is defined
	/// - if behaviors or subtrees are missing
	pub fn create_from_text(&mut self, xml: &str) -> Result<BehaviorTree, Error> {
		self.register_behavior_tree_from_text(xml)?;
		self.create_main_tree()
	}

	/// Create a [`BehaviorTree`] from previous registration.
	/// # Errors
	/// - if no main tree has been defined during regisration
	/// - if behaviors or subtrees are missing
	pub fn create_main_tree(&mut self) -> Result<BehaviorTree, Error> {
		if let Some(name) = self.main_tree_name.clone() {
			self.create_tree(&name)
		} else {
			self.create_tree("MainTree")
		}
	}

	/// Create the named [`BehaviorTree`] from registration
	/// # Errors
	/// - if no tree with `name` can be found
	/// - if behaviors or subtrees are missing
	pub fn create_tree(&mut self, name: &str) -> Result<BehaviorTree, Error> {
		let mut parser = XmlParser::default();
		let root = parser.create_tree_from_definition(name, &mut self.registry)?;
		Ok(BehaviorTree::new(root, &self.registry))
	}

	/// Prints out the list of registered behaviors.
	#[cfg(feature = "std")]
	pub fn list_behaviors(&self) {
		self.registry.list_behaviors();
	}

	/// Register the behavior (sub)trees described by the XML.
	/// # Errors
	/// - on incorrect XML
	/// - if tree description is not in BTCPP v4
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

		// handle the attribute 'main_tree_to_execute`
		self.main_tree_name = root
			.attribute("main_tree_to_execute")
			.map(Into::into);

		XmlParser::register_document_root(&mut self.registry, root)?;
		Ok(())
	}

	/// Get the name list of registered behavior trees.
	#[must_use]
	pub fn registered_behavior_trees(&self) -> Vec<ConstString> {
		self.registry.registered_behavior_trees()
	}

	/// Register a behavior plugin.
	/// For now it is  recommended, that
	/// - the plugin resides in the executables directory and
	/// - is compiled with the same `Rust` version.
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
		// Therefore the library is handed over to the behavior registry and later referenced by any tree.
		self.registry.add_library(lib);
		Ok(())
	}

	/// Register a `Behavior` of type `<T>`.
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_node_type<T>(&mut self, name: &str) -> Result<(), Error>
	where
		T: Behavior,
	{
		let bhvr_creation_fn = T::creation_fn();
		let bhvr_type = T::kind();
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`BehaviorType::Action`].
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_action(&mut self, name: &str, tick_fn: SimpleBhvrTickFn) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = BehaviorType::Action;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`BehaviorType::Action`] which is using ports.
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_action_with_ports(
		&mut self,
		name: &str,
		tick_fn: ComplexBhvrTickFn,
		port_list: PortList,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::new_create_with_ports(tick_fn, port_list);
		let bhvr_type = BehaviorType::Action;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`BehaviorType::Condition`].
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_condition(&mut self, name: &str, tick_fn: SimpleBhvrTickFn) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		let bhvr_type = BehaviorType::Condition;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}

	/// Register a function as [`BehaviorType::Condition`] which is using ports.
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_condition_with_ports(
		&mut self,
		name: &str,
		tick_fn: ComplexBhvrTickFn,
		port_list: PortList,
	) -> Result<(), Error> {
		let bhvr_creation_fn = SimpleBehavior::new_create_with_ports(tick_fn, port_list);
		let bhvr_type = BehaviorType::Condition;
		self.registry
			.add_behavior(name, bhvr_creation_fn, bhvr_type)
	}
}
// endregion:   --- BehaviorTreeFactory
