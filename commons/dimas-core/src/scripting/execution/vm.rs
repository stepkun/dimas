// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Virtual machine for `DiMAS` scripting

extern crate std;

use core::marker::PhantomData;

use crate::scripting::{Chunk, error::Error};

#[allow(clippy::wildcard_imports)]
use super::opcodes::*;
use super::values::{Numbers, Value};

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
	fn reset(&mut self) {
		self.ip = 0;
		self.stack_top = 0;
	}

	fn push(&mut self, value: Value) {
		self.stack[self.stack_top] = value;
		self.stack_top += 1;
	}

	fn pop(&mut self) -> Value {
		self.stack_top -= 1;
		self.stack[self.stack_top]
	}

	/// Execute a [`Chunk`] with the virtual machine
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(&mut self, chunk: &Chunk) -> Result<(), Error> {
		self.reset();
		// ignore empty chunks
		if chunk.code().is_empty() {
			return Ok(());
		};
		loop {
			let instruction: u8 = chunk.code()[self.ip];
			self.ip += 1;
			match instruction {
				OP_RETURN => {
					std::println!("{}", self.pop());
					return Ok(());
				}
				OP_ADD => {
					let b = self.pop();
					let a = self.pop();
					self.push(a + b);
				}
				OP_SUBTRACT => {
					let b = self.pop();
					let a = self.pop();
					self.push(a - b);
				}
				OP_MULTIPLY => {
					let b = self.pop();
					let a = self.pop();
					self.push(a * b);
				}
				OP_DIVIDE => {
					let b = self.pop();
					let a = self.pop();
					self.push(a / b);
				}
				OP_NEGATE => {
					let tmp = -self.pop();
					self.push(tmp);
				}
				OP_CONSTANT => {
					let pos = chunk.code()[self.ip];
					let constant = chunk.read_constant(pos);
					self.ip += 1;
					self.push(constant);
				}
				_ => return Err(Error::UnknownOpCode),
			}
		}
	}
}
