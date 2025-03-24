// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Virtual machine for `DiMAS` scripting

extern crate std;

use core::{marker::PhantomData, str::CharIndices};

use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
};
use hashbrown::HashSet;

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
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack: [Value::default(); STACK_MAX],
			stack_top: 0,
		}
	}
}

impl VM {
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
		let b_kind = self.peek(0).kind();
		let a_kind = self.peek(1).kind();
		// Strings can be concatenated with
		if a_kind == VAL_STR {
			match operator {
				OP_ADD => {
					let b = self.pop();
					let a_pos = self.pop().as_string_pos()?;
					let a = chunk
						.get_string(a_pos)
						.trim_matches('\'')
						.to_owned();
					let res = match b.kind() {
						VAL_BOOL => {
							let b = b.as_bool()?;
							a + &b.to_string()
						}
						VAL_DOUBLE => {
							let b = b.as_double()?;
							a + &b.to_string()
						}
						VAL_INT => {
							let b = b.as_integer()?;
							a + &b.to_string()
						}
						VAL_NIL => a + "nil",
						VAL_STR => {
							let b_pos = b.as_string_pos()?;
							a + chunk.get_string(b_pos).trim_matches('\'')
						}
						_ => return Err(Error::Unreachable),
					};
					let string_pos = chunk.add_string(res);
					let value = Value::from_string_pos(string_pos);
					self.push(value);
					Ok(())
				}
				_ => Err(Error::OnlyAdd),
			}
		} else if b_kind == a_kind && (b_kind == VAL_DOUBLE || b_kind == VAL_INT) {
			let mut b_val = self.pop();
			let mut a_val = self.pop();
			if b_kind == VAL_DOUBLE {
				let b = b_val.as_double()?;
				let a = a_val.as_double()?;
				let res = match operator {
					OP_ADD => a + b,
					OP_SUBTRACT => a - b,
					OP_MULTIPLY => a * b,
					OP_DIVIDE => a / b,
					_ => return Err(Error::Unreachable),
				};
				a_val.to_double(res);
			} else {
				let b = b_val.as_integer()?;
				let a = a_val.as_integer()?;
				let res = match operator {
					OP_ADD => a + b,
					OP_SUBTRACT => a - b,
					OP_MULTIPLY => a * b,
					OP_DIVIDE => a / b,
					_ => return Err(Error::Unreachable),
				};
				a_val.to_integer(res);
			}
			self.push(a_val);
			Ok(())
		} else {
			Err(Error::NoNumber)
		}
	}

	fn boolean_operator(&mut self, operator: u8) -> Result<(), Error> {
		let b_kind = self.peek(0).kind();
		let a_kind = self.peek(1).kind();
		if b_kind == a_kind && (b_kind == VAL_DOUBLE || b_kind == VAL_INT) {
			let mut b_val = self.pop();
			let mut a_val = self.pop();
			if b_kind == VAL_DOUBLE {
				let b = b_val.as_double()?;
				let a = a_val.as_double()?;
				let res = match operator {
					OP_GREATER => a > b,
					OP_LESS => a < b,
					_ => return Err(Error::Unreachable),
				};
				a_val.to_bool(res);
			} else {
				let b = b_val.as_integer()?;
				let a = a_val.as_integer()?;
				let res = match operator {
					OP_GREATER => a > b,
					OP_LESS => a < b,
					_ => return Err(Error::Unreachable),
				};
				a_val.to_bool(res);
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
		let b = self.pop();
		let a = self.pop();
		if a.kind() == b.kind() {
			let res = match a.kind() {
				VAL_BOOL => a.as_bool().expect("snh") == b.as_bool().expect("snh"),
				#[allow(clippy::float_cmp)]
				VAL_DOUBLE => {
					let epsilon = 0.000_000_000_000_002;
					let delta = f64::abs(a.as_double().expect("snh") - b.as_double().expect("snh"));
					delta <= epsilon
				}
				VAL_INT => a.as_integer().expect("snh") == b.as_integer().expect("snh"),
				VAL_STR => {
					let a_pos = a.as_string_pos().expect("snh");
					let b_pos = b.as_string_pos().expect("snh");
					let a = chunk.get_string(a_pos);
					let b = chunk.get_string(b_pos);
					a == b
				}
				VAL_NIL => true,
				_ => false,
			};
			self.push(Value::from_bool(res));
		} else {
			self.push(Value::from_bool(false));
		}
	}

	fn negate(&mut self) -> Result<(), Error> {
		let val_kind = self.peek(0).kind();
		if val_kind == VAL_DOUBLE || val_kind == VAL_INT {
			let mut val = self.pop();
			if val_kind == VAL_DOUBLE {
				let double = -val.as_double()?;
				val.to_double(double);
			} else {
				let integer = -val.as_integer()?;
				val.to_integer(integer);
			}
			self.push(val);
			Ok(())
		} else {
			Err(Error::NoNumber)
		}
	}

	fn binary_not(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let kind = self.peek(0).kind();
		if kind != VAL_INT {
			return Err(Error::NoInteger);
		}
		let mut val = self.pop();
		val.to_integer(!val.as_integer()?);
		self.push(val);
		Ok(())
	}

	fn not(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let kind = self.peek(0).kind();
		let mut val = self.pop();
		match kind {
			VAL_BOOL => {
				val.to_bool(!val.as_bool()?);
			}
			VAL_DOUBLE | VAL_STR | VAL_INT => {
				val.to_bool(false);
			}
			VAL_NIL => {
				val.to_bool(true);
			}
			_ => return Err(Error::Unreachable),
		}
		self.push(val);
		Ok(())
	}

	/// Execute a [`Chunk`] with the virtual machine
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(
		&mut self,
		chunk: &mut Chunk,
		stdout: &mut impl std::io::Write,
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
				OP_DIVIDE => self.arithmetic_operator(OP_DIVIDE, chunk)?,
				OP_EQUAL => self.equal(chunk),
				OP_FALSE => self.push(Value::from_bool(false))?,
				OP_GREATER => self.boolean_operator(OP_GREATER)?,
				OP_LESS => self.boolean_operator(OP_LESS)?,
				OP_MULTIPLY => self.arithmetic_operator(OP_MULTIPLY, chunk)?,
				OP_NEGATE => self.negate()?,
				OP_NIL => self.push(Value::nil())?,
				OP_NOT => self.not(chunk)?,
				OP_POP => {
					self.pop();
				}
				OP_PRINT => {
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
				}
				OP_RETURN => {
					chunk.restore_state();
					return Ok(());
				}
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
