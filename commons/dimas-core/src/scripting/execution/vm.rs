// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Virtual machine for `DiMAS` scripting

extern crate std;

use core::marker::PhantomData;

#[allow(clippy::wildcard_imports)]
use super::opcodes::*;
use super::{
	error::Error, values::{Value, BOOLEAN, DOUBLE, INTEGER, NIL}, Chunk
};

/// Stack size is fixed
const STACK_MAX: usize = 256;

/// A Virtual Machine
pub struct VM {
	ip: usize,
	stack: [Value; STACK_MAX],
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
	const fn reset(&mut self) {
		self.ip = 0;
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

	fn arithmetic_operator(&mut self, operator: u8) -> Result<(), Error> {
		let b_kind = self.peek(0).kind();
		let a_kind = self.peek(1).kind();
		if b_kind == a_kind && (b_kind == DOUBLE || b_kind == INTEGER) {
			let mut b_val = self.pop();
			let mut a_val = self.pop();
			if b_kind == DOUBLE {
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
		if b_kind == a_kind && (b_kind == DOUBLE || b_kind == INTEGER) {
			let mut b_val = self.pop();
			let mut a_val = self.pop();
			if b_kind == DOUBLE {
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

	fn equal(&mut self, chunk: &Chunk) -> bool {
		let b = self.pop();
		let a = self.pop();
		if a.kind() == b.kind() {
			match a.kind() {
				BOOLEAN => a.as_bool().expect("snh") == b.as_bool().expect("snh"),
				#[allow(clippy::float_cmp)]	// @TODO: define an epsilon
				DOUBLE => a.as_double().expect("snh") == b.as_double().expect("snh"),
				INTEGER => a.as_integer().expect("snh") == b.as_integer().expect("snh"),
				NIL => true,
				_ => false,
			}
		} else {
			false
		}
	}

	fn negate(&mut self) -> Result<(), Error> {
		let val_kind = self.peek(0).kind();
		if val_kind == DOUBLE || val_kind == INTEGER {
			let mut val = self.pop();
			if val_kind == DOUBLE {
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

	fn not(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let kind = self.peek(0).kind();
		if kind != BOOLEAN && kind != NIL {
			return Err(Error::NoBoolean)
		}
		// 'nil' will be left untouched
		if kind == BOOLEAN {
			let mut val = self.pop();
			val.to_bool(!val.as_bool()?);
			self.push(val);
		}
		Ok(())
	}

	/// Execute a [`Chunk`] with the virtual machine
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(&mut self, chunk: &Chunk) -> Result<(), Error> {
		self.reset();
		// ignore empty chunks
		if chunk.code().is_empty() {
			return Ok(());
		}

		loop {
			let instruction: u8 = chunk.code()[self.ip];
			self.ip += 1;
			match instruction {
				OP_ADD => self.arithmetic_operator(OP_ADD)?,
				OP_CONSTANT => self.constant(chunk),
				OP_DIVIDE => self.arithmetic_operator(OP_DIVIDE)?,
				OP_EQUAL => {
					let res = self.equal(chunk); 
					self.push(Value::from_bool(res));
				},
				OP_FALSE => self.push(Value::from_bool(false))?,
				OP_GREATER => self.boolean_operator(OP_GREATER)?,
				OP_LESS => self.boolean_operator(OP_LESS)?,
				OP_MULTIPLY => self.arithmetic_operator(OP_MULTIPLY)?,
				OP_NEGATE => self.negate()?,
				OP_NIL => self.push(Value::nil())?,
				OP_NOT => self.not(chunk)?,
				OP_RETURN => {
					if self.stack_top > 0 {
						std::println!("{}", self.pop());
					} else {
						std::println!("no result");
					}
					return Ok(());
				}
				OP_SUBTRACT => self.arithmetic_operator(OP_SUBTRACT)?,
				OP_TRUE => self.push(Value::from_bool(true))?,
				_ => return Err(Error::UnknownOpCode),
			}
		}
	}
}
