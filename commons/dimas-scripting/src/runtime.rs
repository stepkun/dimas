// Copyright © 2025 Stephan Kunz

//! Runtime environment for `DiMAS` scripting
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
#[cfg(feature = "std")]
use alloc::vec::Vec;
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};
use parking_lot::Mutex;

use crate::{
	Environment,
	compiling::Parser,
	error::Error,
	execution::{Chunk, ScriptingValue, VM},
};
// endregion:   --- modules

// region:      --- types
/// Definition of a shared [`Runtime`].
pub type SharedRuntime = Arc<Mutex<Runtime>>;
// endregion:   --- types

// region:      --- Runtime
/// Runtime environment for scripting.
#[derive(Debug, Default)]
pub struct Runtime {
	parser: Parser,
	vm: VM,
	enums: BTreeMap<String, i8>,
	#[cfg(feature = "std")]
	stdout: Vec<u8>,
}

/// Cloning a Runtime is cloning the environment.
/// Parser, VM and stdout are created new.
impl Clone for Runtime {
	fn clone(&self) -> Self {
		Self {
			parser: Parser::default(),
			vm: VM::default(),
			enums: self.enums.clone(),
			#[cfg(feature = "std")]
			stdout: Vec::new(),
		}
	}
}

impl Runtime {
	/// Insert enum value.
	///
	/// # Errors
	/// - if en enum definition (key) already exists.
	pub fn register_enum_tuple(&mut self, key: &str, value: i8) -> Result<(), Error> {
		if let Some(old_value) = self.enums.get(key) {
			return Err(Error::DuplicateEnumVariant(key.into(), *old_value, value));
		}
		self.enums.insert(key.into(), value);
		Ok(())
	}

	/// Parse a scripting source.
	/// # Errors
	/// - if script is invalid
	pub fn parse(&mut self, script: &str) -> Result<Chunk, Error> {
		self.parser.parse(&self.enums, script)
	}

	/// Execute a bytecode chunk.
	/// # Errors
	/// - if
	pub fn execute(&mut self, chunk: &Chunk, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(chunk, globals);
		#[cfg(feature = "std")]
		let res = self.vm.run(chunk, globals, &mut self.stdout);
		res
	}

	/// Continue running scripts.
	/// Does not clear stdout before execution.
	/// # Errors
	/// - if
	pub fn continue_run(&mut self, script: &str, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		let chunk = self.parser.parse(&self.enums, script)?;
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(chunk, globals)?;
		#[cfg(feature = "std")]
		let res = self.vm.run(&chunk, globals, &mut self.stdout)?;
		Ok(res)
	}

	/// Run a script.
	/// Clears stdout before execution.
	/// # Errors
	/// - if
	pub fn run(&mut self, script: &str, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		self.stdout.clear();
		self.continue_run(script, globals)
	}

	/// Access the stdout.
	#[cfg(feature = "std")]
	#[must_use]
	pub const fn stdout(&self) -> &Vec<u8> {
		&self.stdout
	}

	/// Clear stdout.
	#[cfg(feature = "std")]
	pub fn clear(&mut self) {
		self.stdout.clear();
	}
}
// endregion:   --- Runtime
