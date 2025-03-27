// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Virtual machine for `DiMAS` scripting

extern crate std;

use core::{marker::PhantomData, str::CharIndices};

use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
	sync::Arc,
};

use crate::{DefaultEnvironment, Environment};

#[allow(clippy::wildcard_imports)]
use super::opcodes::*;
use super::{
	Chunk, chunk,
	error::Error,
	values::{VAL_BOOL, VAL_DOUBLE, VAL_INT, VAL_NIL, VAL_STR, Value},
};

/// Stack size is fixed
const STACK_MAX: usize = 256;

/// A Virtual Machine
pub struct VM {
	/// The `InstructionPointer` (sometimes called `ProgramCounter`)
	ip: usize,
	/// Stack for values
	stack: [Value; STACK_MAX],
	/// Pointer to the next free stack place
	stack_top: usize,
	/// Reference to storage for truly `global` variables, which are used also available outside the [`VM`]
	globals: Arc<dyn Environment>,
}

impl Default for VM {
	/// Create a [`VM`] with a default Environment
	fn default() -> Self {
		Self {
			ip: 0,
			stack: [Value::default(); STACK_MAX],
			stack_top: 0,
			globals: Arc::from(DefaultEnvironment::default()),
		}
	}
}

impl VM {
	/// Create a [`VM`] with an external Environment
	pub fn new(environment: Arc<dyn Environment>) -> Self {
		Self {
			ip: 0,
			stack: [Value::default(); STACK_MAX],
			stack_top: 0,
			globals: environment,
		}
	}

	fn reset(&mut self) {
		self.ip = 0;
		self.stack = [Value::default(); STACK_MAX];
		self.stack_top = 0;
	}

	const fn peek(&self, distance: usize) -> &Value {
		&self.stack[self.stack_top - distance - 1]
	}

	const fn push(&mut self, value: Value) -> Result<(), Error> {
		if self.stack_top == u8::MAX as usize {
			return Err(Error::StackOverflow);
		}
		self.stack[self.stack_top] = value;
		self.stack_top += 1;
		Ok(())
	}

	const fn pop(&mut self) -> Value {
		self.stack_top -= 1;
		self.stack[self.stack_top]
	}

	fn arithmetic_operator(&mut self, operator: u8, chunk: &mut Chunk) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		let b_kind = b_val.kind();
		let a_kind = a_val.kind();
		// Strings can be concatenated with
		if a_kind == VAL_STR {
			match operator {
				OP_ADD => {
					let a_pos = a_val.as_string_pos()?;
					let a = chunk.get_string(a_pos).to_owned();
					let res = match b_kind {
						VAL_BOOL => {
							let b = b_val.as_bool()?;
							a + &b.to_string()
						}
						VAL_DOUBLE => {
							let b = b_val.as_double()?;
							a + &b.to_string()
						}
						VAL_INT => {
							let b = b_val.as_integer()?;
							a + &b.to_string()
						}
						VAL_NIL => a + "nil",
						VAL_STR => {
							let b_pos = b_val.as_string_pos()?;
							a + chunk.get_string(b_pos)
						}
						_ => return Err(Error::Unreachable(line!())),
					};
					let string_pos = chunk.add_string(res);
					a_val.make_string_pos(string_pos);
					self.push(a_val);
					Ok(())
				}
				_ => Err(Error::OnlyAdd),
			}
		} else if b_kind == a_kind && (b_kind == VAL_DOUBLE || b_kind == VAL_INT) {
			if b_kind == VAL_DOUBLE {
				let b = b_val.as_double()?;
				let a = a_val.as_double()?;
				let res = match operator {
					OP_ADD => a + b,
					OP_SUBTRACT => a - b,
					OP_MULTIPLY => a * b,
					OP_DIVIDE => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val.make_double(res);
			} else {
				let b = b_val.as_integer()?;
				let a = a_val.as_integer()?;
				let res = match operator {
					OP_ADD => a + b,
					OP_SUBTRACT => a - b,
					OP_MULTIPLY => a * b,
					OP_DIVIDE => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val.make_integer(res);
			}
			self.push(a_val);
			Ok(())
		} else {
			Err(Error::NoNumber)
		}
	}

	fn boolean_operator(&mut self, operator: u8) -> Result<(), Error> {
		let mut b_val = self.pop();
		let mut a_val = self.pop();
		let b_kind = b_val.kind();
		let a_kind = a_val.kind();
		if b_kind == a_kind && (b_kind == VAL_DOUBLE || b_kind == VAL_INT) {
			if b_kind == VAL_DOUBLE {
				let res = match operator {
					OP_GREATER => a_val.as_double()? > b_val.as_double()?,
					OP_LESS => a_val.as_double()? < b_val.as_double()?,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val.make_bool(res);
			} else {
				let res = match operator {
					OP_GREATER => a_val.as_integer()? > b_val.as_integer()?,
					OP_LESS => a_val.as_integer()? < b_val.as_integer()?,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val.make_bool(res);
			}
			self.push(a_val);
			Ok(())
		} else {
			Err(Error::NoNumber)
		}
	}

	fn constant(&mut self, chunk: &Chunk) {
		let pos = chunk.code()[self.ip];
		let constant = chunk.read_constant(pos);
		self.ip += 1;
		self.push(constant);
	}

	fn equal(&mut self, chunk: &Chunk) {
		let b_val = self.pop();
		let mut a_val = self.pop();
		let a_kind = a_val.kind();
		if a_kind == b_val.kind() {
			let res = match a_kind {
				VAL_BOOL => a_val.as_bool().expect("snh") == b_val.as_bool().expect("snh"),
				VAL_DOUBLE => {
					let delta =
						f64::abs(a_val.as_double().expect("snh") - b_val.as_double().expect("snh"));
					delta <= 0.000_000_000_000_002
				}
				VAL_INT => a_val.as_integer().expect("snh") == b_val.as_integer().expect("snh"),
				VAL_STR => {
					let a_pos = a_val.as_string_pos().expect("snh");
					let a = chunk.get_string(a_pos);
					let b_pos = b_val.as_string_pos().expect("snh");
					let b = chunk.get_string(b_pos);
					a == b
				}
				VAL_NIL => true,
				_ => false,
			};
			a_val.make_bool(res);
		} else {
			a_val.make_bool(false);
		}
		self.push(a_val);
	}

	fn negate(&mut self) -> Result<(), Error> {
		let mut val = self.pop();
		let val_kind = val.kind();
		if val_kind == VAL_DOUBLE || val_kind == VAL_INT {
			if val_kind == VAL_DOUBLE {
				let double = -val.as_double()?;
				val.make_double(double);
			} else {
				let integer = -val.as_integer()?;
				val.make_integer(integer);
			}
			self.push(val);
			Ok(())
		} else {
			Err(Error::NoNumber)
		}
	}

	fn binary_not(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let mut val = self.pop();
		let kind = val.kind();
		if kind != VAL_INT {
			return Err(Error::NoInteger);
		}
		val.make_integer(!val.as_integer()?);
		self.push(val);
		Ok(())
	}

	fn not(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let mut val = self.pop();
		let kind = val.kind();
		match kind {
			VAL_BOOL => {
				val.make_bool(!val.as_bool()?);
			}
			VAL_DOUBLE | VAL_STR | VAL_INT => {
				val.make_bool(false);
			}
			VAL_NIL => {
				val.make_bool(true);
			}
			_ => return Err(Error::Unreachable(line!())),
		}
		self.push(val);
		Ok(())
	}

	#[cfg(feature = "std")]
	fn print(&mut self, chunk: &Chunk, stdout: &mut impl std::io::Write) -> Result<(), Error> {
		if self.stack_top > 0 {
			let value = self.pop();
			if value.is_string_pos() {
				std::writeln!(stdout, "{}", chunk.get_string(value.as_string_pos()?));
			} else {
				std::writeln!(stdout, "{value}");
			}
		} else {
			std::writeln!(stdout, "no result");
		}
		Ok(())
	}

	fn define_global(&mut self, chunk: &Chunk) -> Result<(), Error> {
		// chunk.disassemble("chunk before define_global");
		let name_val = self.pop();
		let value_val = self.pop();
		let name = chunk.get_string(name_val.as_string_pos()?);
		self.globals.define(name, value_val);
		Ok(())
	}

	fn get_global(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let name_val = self.pop();
		let name = chunk.get_string(name_val.as_string_pos()?);
		let val = self.globals.get(name)?;
		self.push(val);
		Ok(())
	}

	fn set_global(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let name_val = self.pop();
		let name = chunk.get_string(name_val.as_string_pos()?);
		let value_val = self.pop();
		self.globals.set(name, value_val)
	}

	/// Execute a [`Chunk`] with the virtual machine
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(
		&mut self,
		chunk: &mut Chunk,
		#[cfg(feature = "std")] stdout: &mut impl std::io::Write,
	) -> Result<(), Error> {
		self.reset();
		chunk.save_state();
		// ignore empty chunks
		if chunk.code().is_empty() {
			chunk.restore_state();
			return Ok(());
		}

		loop {
			let instruction: u8 = chunk.code()[self.ip];
			self.ip += 1;
			match instruction {
				OP_ADD => self.arithmetic_operator(OP_ADD, chunk)?,
				OP_BINARY_NOT => self.binary_not(chunk)?,
				OP_CONSTANT => self.constant(chunk),
				OP_DEFINE_EXTERNAL => self.define_global(chunk)?,
				OP_DIVIDE => self.arithmetic_operator(OP_DIVIDE, chunk)?,
				OP_EQUAL => self.equal(chunk),
				OP_FALSE => self.push(Value::from_bool(false))?,
				OP_GET_EXTERNAL => self.get_global(chunk)?,
				OP_GREATER => self.boolean_operator(OP_GREATER)?,
				OP_LESS => self.boolean_operator(OP_LESS)?,
				OP_MULTIPLY => self.arithmetic_operator(OP_MULTIPLY, chunk)?,
				OP_NEGATE => self.negate()?,
				OP_NIL => self.push(Value::nil())?,
				OP_NOT => self.not(chunk)?,
				OP_POP => {
					self.pop();
				}
				#[cfg(feature = "std")]
				OP_PRINT => self.print(chunk, stdout)?,
				OP_RETURN => {
					chunk.restore_state();
					return Ok(());
				}
				OP_SET_EXTERNAL => self.set_global(chunk)?,
				OP_SUBTRACT => self.arithmetic_operator(OP_SUBTRACT, chunk)?,
				OP_TRUE => self.push(Value::from_bool(true))?,
				_ => {
					chunk.restore_state();
					return Err(Error::UnknownOpCode);
				}
			}
		}
		chunk.restore_state();
	}
}
