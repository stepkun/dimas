// Copyright © 2025 Stephan Kunz

//! Factory for creation and modification of [`BehaviorTree`]s.
//!
//! The factory ensures that a tree is properly created and libraries or plugins
//! are loaded properly and kept in memory as long as needed.

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use dimas_core::ConstString;
use roxmltree::Document;

use crate::{
	behavior::{
		Behavior, BehaviorDescription, BehaviorExecution, BehaviorKind, BehaviorState, BehaviorStatic, ComplexBhvrTickFn,
		SimpleBehavior, SimpleBhvrTickFn,
		action::{ChangeStateAfter, Script, SetBlackboard, Sleep, UnsetBlackboard},
		condition::{ScriptCondition, WasEntryUpdated},
		control::{
			Fallback, IfThenElse, Parallel, ParallelAll, ReactiveFallback, ReactiveSequence, Sequence, SequenceWithMemory,
			Switch, WhileDoElse,
		},
		decorator::{
			Delay, EntryUpdated, ForceState, Inverter, KeepRunningUntilFailure, Loop, Precondition, Repeat,
			RetryUntilSuccessful, RunOnce, Subtree, Timeout,
		},
	},
	blackboard::SharedBlackboard,
	port::PortList,
	tree::BehaviorTree,
	xml::parser::XmlParser,
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- BehaviorTreeFactory
/// Factory for creation and modification of [`BehaviorTree`]s
pub struct BehaviorTreeFactory {
	registry: BehaviorRegistry,
}

impl Default for BehaviorTreeFactory {
	fn default() -> Self {
		let mut f = Self {
			registry: BehaviorRegistry::default(),
		};
		// minimum required behaviors for the factory to work
		f.register_groot2_behavior_type::<Subtree>("SubTree")
			.expect("snh");
		f
	}
}

impl BehaviorTreeFactory {
	/// Access the registry.
	#[must_use]
	pub const fn registry(&self) -> &BehaviorRegistry {
		&self.registry
	}

	/// Access the registry mutable.
	#[must_use]
	pub const fn registry_mut(&mut self) -> &mut BehaviorRegistry {
		&mut self.registry
	}

	/// Create a factory with core set of behaviors
	/// # Errors
	/// - if behaviors cannot be registered
	pub fn with_core_behaviors() -> Result<Self, Error> {
		let mut factory = Self::default();
		factory.core_behaviors()?;
		if cfg!(test) {
			factory.test_behaviors()?;
		}
		Ok(factory)
	}

	/// Create a factory with extended set of behaviors
	/// # Errors
	/// - if behaviors cannot be registered
	pub fn with_extended_behaviors() -> Result<Self, Error> {
		let mut factory = Self::with_core_behaviors()?;
		factory.extended_behaviors()?;
		Ok(factory)
	}

	/// Create a factory with groot2 builtin behaviors
	/// # Errors
	/// - if behaviors cannot be registered
	pub fn with_groot2_behaviors() -> Result<Self, Error> {
		let mut factory = Self::with_extended_behaviors()?;
		if !cfg!(test) {
			factory.test_behaviors()?;
		}
		factory.groot2_behaviors()?;
		Ok(factory)
	}

	/// register core behaviors
	/// # Errors
	/// - if any registration fails
	pub fn core_behaviors(&mut self) -> Result<(), Error> {
		// actions
		self.register_groot2_behavior_type::<Script>("Script")?;

		// conditions
		self.register_groot2_behavior_type::<ScriptCondition>("ScriptCondition")?;
		self.register_groot2_behavior_type::<WasEntryUpdated>("WasEntryUpdated")?;

		// controls
		self.register_groot2_behavior_type::<Fallback>("Fallback")?;
		self.register_groot2_behavior_type::<Parallel>("Parallel")?;
		self.register_groot2_behavior_type::<Sequence>("Sequence")?;

		// decorators
		self.register_groot2_behavior_type::<Inverter>("Inverter")?;

		let bhvr_desc = BehaviorDescription::new(
			"SkipUnlessUpdated",
			"SkipUnlessUpdated",
			EntryUpdated::kind(),
			true,
			EntryUpdated::provided_ports(),
		);
		let bhvr_creation_fn =
			Box::new(move || -> Box<dyn BehaviorExecution> { Box::new(EntryUpdated::new(BehaviorState::Skipped)) });
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		let bhvr_desc = BehaviorDescription::new(
			"WaitValueUpdated",
			"WaitValueUpdated",
			EntryUpdated::kind(),
			true,
			EntryUpdated::provided_ports(),
		);
		let bhvr_creation_fn =
			Box::new(move || -> Box<dyn BehaviorExecution> { Box::new(EntryUpdated::new(BehaviorState::Running)) });
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		Ok(())
	}

	/// register test behaviors
	/// # Errors
	/// - if any registration fails
	pub fn test_behaviors(&mut self) -> Result<(), Error> {
		// actions
		let bhvr_desc = BehaviorDescription::new(
			"AlwaysFailure",
			"AlwaysFailure",
			ChangeStateAfter::kind(),
			true,
			ChangeStateAfter::provided_ports(),
		);
		let bhvr_creation_fn = Box::new(move || -> Box<dyn BehaviorExecution> {
			Box::new(ChangeStateAfter::new(BehaviorState::Running, BehaviorState::Failure, 0))
		});
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		let bhvr_desc = BehaviorDescription::new(
			"AlwaysRunning",
			"AlwaysRunning",
			ChangeStateAfter::kind(),
			false,
			ChangeStateAfter::provided_ports(),
		);
		let bhvr_creation_fn = Box::new(move || -> Box<dyn BehaviorExecution> {
			Box::new(ChangeStateAfter::new(BehaviorState::Running, BehaviorState::Running, 0))
		});
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		let bhvr_desc = BehaviorDescription::new(
			"AlwaysSuccess",
			"AlwaysSuccess",
			ChangeStateAfter::kind(),
			true,
			ChangeStateAfter::provided_ports(),
		);
		let bhvr_creation_fn = Box::new(move || -> Box<dyn BehaviorExecution> {
			Box::new(ChangeStateAfter::new(BehaviorState::Running, BehaviorState::Success, 0))
		});
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		// conditions

		// controls

		// decorators
		let bhvr_desc = BehaviorDescription::new(
			"ForceFailure",
			"ForceFailure",
			ForceState::kind(),
			true,
			ForceState::provided_ports(),
		);
		let bhvr_creation_fn =
			Box::new(move || -> Box<dyn BehaviorExecution> { Box::new(ForceState::new(BehaviorState::Failure)) });
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		let bhvr_desc = BehaviorDescription::new(
			"ForceSuccess",
			"ForceSuccess",
			ForceState::kind(),
			true,
			ForceState::provided_ports(),
		);
		let bhvr_creation_fn =
			Box::new(move || -> Box<dyn BehaviorExecution> { Box::new(ForceState::new(BehaviorState::Success)) });
		self.registry_mut()
			.add_behavior(bhvr_desc, bhvr_creation_fn)?;

		Ok(())
	}

	/// register extended behaviors
	/// # Errors
	/// - if any registration fails
	pub fn extended_behaviors(&mut self) -> Result<(), Error> {
		// actions
		self.register_groot2_behavior_type::<Sleep>("Sleep")?;

		// conditions

		// controls
		self.register_groot2_behavior_type::<IfThenElse>("IfThenElse")?;
		self.register_groot2_behavior_type::<ParallelAll>("ParallelAll")?;
		self.register_groot2_behavior_type::<ReactiveFallback>("ReactiveFallback")?;
		self.register_groot2_behavior_type::<ReactiveSequence>("ReactiveSequence")?;
		self.register_groot2_behavior_type::<SequenceWithMemory>("SequenceWithMemory")?;
		self.register_groot2_behavior_type::<WhileDoElse>("WhileDoElse")?;

		// decorators
		self.register_groot2_behavior_type::<Delay>("Delay")?;
		self.register_groot2_behavior_type::<KeepRunningUntilFailure>("KeepRunningUntilFailure")?;
		self.register_groot2_behavior_type::<Repeat>("Repeat")?;
		self.register_groot2_behavior_type::<RetryUntilSuccessful>("RetryUntilSuccessful")?;
		self.register_groot2_behavior_type::<RunOnce>("RunOnce")?;
		self.register_groot2_behavior_type::<Timeout>("Timeout")?;

		Ok(())
	}

	/// register groot2 builtin behaviors
	/// # Errors
	/// - if any registration fails
	pub fn groot2_behaviors(&mut self) -> Result<(), Error> {
		// actions
		self.register_groot2_behavior_type::<SetBlackboard<String>>("SetBlackboard")?;
		self.register_groot2_behavior_type::<UnsetBlackboard<String>>("UnsetBlackboard")?;

		// conditions

		// controls
		self.register_groot2_behavior_type::<Fallback>("AsyncFallback")?;
		self.register_groot2_behavior_type::<Sequence>("AsyncSequence")?;
		self.register_groot2_behavior_type::<Switch<2>>("Switch2")?;
		self.register_groot2_behavior_type::<Switch<3>>("Switch3")?;
		self.register_groot2_behavior_type::<Switch<4>>("Switch4")?;
		self.register_groot2_behavior_type::<Switch<5>>("Switch5")?;
		self.register_groot2_behavior_type::<Switch<6>>("Switch6")?;

		// decorators
		self.register_groot2_behavior_type::<Loop<i32>>("LoopInt")?;
		self.register_groot2_behavior_type::<Loop<bool>>("LoopBool")?;
		self.register_groot2_behavior_type::<Loop<f64>>("LoopDouble")?;
		self.register_groot2_behavior_type::<Loop<String>>("LoopString")?;
		self.register_groot2_behavior_type::<Precondition>("Precondition")?;

		Ok(())
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
		if let Some(name) = self.registry.main_tree_id() {
			self.create_tree(&name)
		} else {
			self.create_tree("MainTree")
		}
	}

	/// Create the named [`BehaviorTree`] from registration.
	/// # Errors
	/// - if no tree with `name` can be found
	/// - if behaviors or subtrees are missing
	pub fn create_tree(&mut self, name: &str) -> Result<BehaviorTree, Error> {
		let mut parser = XmlParser::default();
		let root = parser.create_tree_from_definition(name, &mut self.registry, None)?;
		Ok(BehaviorTree::new(root, &self.registry))
	}

	/// Create the named [`BehaviorTree`] from registration using external created blackboard.
	/// # Errors
	/// - if no tree with `name` can be found
	/// - if behaviors or subtrees are missing
	pub fn create_tree_with(&mut self, name: &str, blackboard: SharedBlackboard) -> Result<BehaviorTree, Error> {
		let mut parser = XmlParser::default();
		let root = parser.create_tree_from_definition(name, &mut self.registry, Some(blackboard))?;
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
		if let Some(name) = root.attribute("main_tree_to_execute") {
			self.registry.set_main_tree_id(name);
		}

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
			let registration_fn: libloading::Symbol<unsafe extern "Rust" fn(&mut Self) -> u32> = lib.get(b"register")?;
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
	pub fn register_behavior_type<T>(&mut self, name: &str) -> Result<(), Error>
	where
		T: Behavior,
	{
		let bhvr_desc = BehaviorDescription::new(name, name, T::kind(), false, T::provided_ports());
		let bhvr_creation_fn = T::creation_fn();
		self.registry
			.add_behavior(bhvr_desc, bhvr_creation_fn)
	}

	/// Register a `Behavior` of type `<T>` which is builtin in Groot2.
	/// # Errors
	/// - if a behavior with that `name` is already registered
	fn register_groot2_behavior_type<T>(&mut self, name: &str) -> Result<(), Error>
	where
		T: Behavior,
	{
		let bhvr_desc = BehaviorDescription::new(name, name, T::kind(), true, T::provided_ports());
		let bhvr_creation_fn = T::creation_fn();
		self.registry
			.add_behavior(bhvr_desc, bhvr_creation_fn)
	}

	/// Register a function either as [`BehaviorType::Action`] or as [`BehaviorType::Condition`].
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_function(
		&mut self,
		name: &str,
		tick_fn: SimpleBhvrTickFn,
		kind: BehaviorKind,
	) -> Result<(), Error> {
		let bhvr_desc = BehaviorDescription::new(name, name, kind, false, PortList::default());
		let bhvr_creation_fn = SimpleBehavior::create(tick_fn);
		self.registry
			.add_behavior(bhvr_desc, bhvr_creation_fn)
	}

	/// Register a function as [`BehaviorType::Action`] or [`BehaviorType::Condition`] which is using ports.
	/// # Errors
	/// - if a behavior with that `name` is already registered
	pub fn register_simple_function_with_ports(
		&mut self,
		name: &str,
		tick_fn: ComplexBhvrTickFn,
		kind: BehaviorKind,
		port_list: PortList,
	) -> Result<(), Error> {
		let bhvr_desc = BehaviorDescription::new(name, name, kind, false, port_list.clone());
		let bhvr_creation_fn = SimpleBehavior::new_create_with_ports(tick_fn, port_list);
		self.registry
			.add_behavior(bhvr_desc, bhvr_creation_fn)
	}
}
// endregion:   --- BehaviorTreeFactory
