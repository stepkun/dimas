// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Bytecode implementation for `DiMAS` scripting

#[doc(hidden)]
extern crate std;

use alloc::{borrow::ToOwned, vec::Vec};

#[allow(clippy::wildcard_imports)]
use crate::scripting::execution::opcodes::*;
use crate::scripting::execution::values::{self, Value};

/// A chunk of bytecode
#[derive(Default)]
pub struct Chunk {
	/// the code
	code: Vec<u8>,
	/// corresponding storage for the line number
	lines: Vec<i16>,
	/// storage for values
	values: Vec<Value>,
}

impl Chunk {
	/// Access code
	#[must_use]
	pub const fn code(&self) -> &Vec<u8> {
		&self.code
	}

	/// Add a byte to the chunk
	pub fn write(&mut self, byte: u8, line: i16) {
		self.code.push(byte);
		self.lines.push(line);
	}

	/// Add a f64 number to the number storage returning its position in the storage
	#[allow(clippy::cast_possible_truncation)]
	pub fn add_constant(&mut self, value: Value) -> u8 {
		self.values.push(value);
		let pos = self.values.len() - 1;
		pos as u8
	}

	/// Read a Value from the Value storage
	#[allow(clippy::redundant_closure_for_method_calls)]
	#[must_use]
	pub fn read_constant(&self, pos: u8) -> Value {
		let offset = usize::from(pos);
		self.values
			.get(offset)
			.map_or_else(|| todo!(), |value| value.to_owned())
	}

	/// Disassemble chunk
	pub fn disassemble(&self, name: &str) {
		let mut offset = 0usize;
		std::println!("== {name} ==");
		while offset < self.code.len() {
			offset = self.disassemble_instruction(offset);
		}
	}

	/// Disassemble a single instruction
	fn disassemble_instruction(&self, offset: usize) -> usize {
		std::print!("{offset:04} ");
		if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
			std::print!("   | ");
		} else {
			std::print!("{:4} ", self.lines[offset]);
		}
		match self.code[offset].to_owned() {
			OP_ADD => Self::simple_instruction("OP_ADD", offset),
			OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
			OP_DIVIDE => Self::simple_instruction("OP_DIVIDE", offset),
			OP_EQUAL => Self::simple_instruction("OP_EQUAL", offset),
			OP_FALSE => Self::simple_instruction("OP_FALSE", offset),
			OP_GREATER => Self::simple_instruction("OP_GREATER", offset),
			OP_LESS => Self::simple_instruction("OP_LESS", offset),
			OP_MULTIPLY => Self::simple_instruction("OP_MULTIPLY", offset),
			OP_NEGATE => Self::simple_instruction("OP_NEGATE", offset),
			OP_NIL => Self::simple_instruction("OP_NIL", offset),
			OP_NOT => Self::simple_instruction("OP_NOT", offset),
			OP_RETURN => Self::simple_instruction("OP_RETURN", offset),
			OP_SUBTRACT => Self::simple_instruction("OP_SUBTRACT", offset),
			OP_TRUE => Self::simple_instruction("OP_TRUE", offset),
			_ => todo!(),
		}
	}

	/// single byte instruction
	fn simple_instruction(name: &str, offset: usize) -> usize {
		std::println!("{name:16}");
		offset + 1
	}

	/// constant instruction
	fn constant_instruction(&self, name: &str, offset: usize) -> usize {
		match self.code.get(offset + 1) {
			Some(pos) => {
				let value = self.read_constant(pos.to_owned());
				match value.kind() {
					values::BOOLEAN => {
						std::println!("{name:16} {pos:3} {}", value.as_bool().expect("snh"));
					}
					values::DOUBLE => {
						std::println!("{name:16} {pos:3} {}", value.as_double().expect("snh"));
					}
					values::INTEGER => {
						std::println!("{name:16} {pos:3} {}", value.as_integer().expect("snh"));
					}
					values::NIL => {
						std::println!("{name:16} {pos:3} 'nil'");
					}
					_ => todo!(),
				}
			}
			None => todo!(),
		}
		offset + 2
	}
}
