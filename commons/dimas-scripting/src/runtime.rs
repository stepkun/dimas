// Copyright © 2025 Stephan Kunz

//! Runtime environment for `DiMAS` scripting
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::sync::Arc;
#[cfg(feature = "std")]
use alloc::vec::Vec;
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
			#[cfg(feature = "std")]
			stdout: Vec::new(),
		}
	}
}

impl Runtime {
	/// Parse a scripting source.
	/// # Errors
	/// - if script is invalid
	pub fn parse(&mut self, script: &str) -> Result<Chunk, Error> {
		self.parser.parse(script)
	}

	/// Execute a bytecode chunk.
	/// # Errors
	/// - if
	pub fn execute(
		&mut self,
		chunk: &Chunk,
		globals: &mut dyn Environment,
	) -> Result<ScriptingValue, Error> {
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(chunk, globals);
		#[cfg(feature = "std")]
		let res = self.vm.run(chunk, globals, &mut self.stdout);
		res
	}

	/// Run a script.
	/// # Errors
	/// - if
	pub fn run(
		&mut self,
		script: &str,
		globals: &mut dyn Environment,
	) -> Result<ScriptingValue, Error> {
		let chunk = self.parser.parse(script)?;
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(chunk, globals)?;
		#[cfg(feature = "std")]
		let res = self.vm.run(&chunk, globals, &mut self.stdout)?;
		Ok(res)
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
