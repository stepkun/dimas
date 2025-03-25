// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Bytecode implementation for `DiMAS` scripting

#[doc(hidden)]
extern crate std;

use alloc::{borrow::ToOwned, string::String, vec::Vec};

#[allow(clippy::wildcard_imports)]
use crate::execution::opcodes::*;
use crate::execution::values::{self, Value};

/// A chunk of bytecode
#[derive(Default)]
pub struct Chunk {
	/// the code
	code: Vec<u8>,
	/// corresponding storage for the line number
	lines: Vec<usize>,
	/// storage for Values
	values: Vec<Value>,
	/// saved values state
	values_state: usize,
	/// storage for Strings
	strings: Vec<String>,
	/// saved strings state
	strings_state: usize,
}

impl Chunk {
	/// Access code
	#[must_use]
	pub const fn code(&self) -> &Vec<u8> {
		&self.code
	}

	/// Save the current size of values and strings
	pub(crate) fn save_state(&mut self) {
		self.values_state = self.values.len();
		self.strings_state = self.strings.len();
	}

	pub(crate) fn restore_state(&mut self) {
		while self.values.len() > self.values_state {
			self.values.pop();
		}
		while self.strings.len() > self.strings_state {
			self.strings.pop();
		}
	}

	/// Add a byte to the chunk
	pub fn write(&mut self, byte: u8, line: usize) {
		self.code.push(byte);
		self.lines.push(line);
	}

	/// Add a Value to the Value storage returning its position in the storage
	#[allow(clippy::cast_possible_truncation)]
	pub fn add_constant(&mut self, value: Value) -> u8 {
		self.values.push(value);
		let pos = self.values.len() - 1;
		pos as u8
	}

	/// Add a String to the String storage returning its position in the storage
	pub fn add_string(&mut self, string: String) -> usize {
		string.trim_matches('\'');
		self.strings.push(string);
		self.strings.len() - 1
	}

	/// Add a String to the Value storage returning its position in the storage
	#[allow(clippy::cast_possible_truncation)]
	pub fn add_string_constant(&mut self, string: String) -> u8 {
		let offset = self.add_string(string);
		let value = Value::from_string_pos(offset);
		self.add_constant(value)
	}

	/// Get the reference to stored [`String`]
	#[must_use]
	pub fn get_string(&self, pos: usize) -> &String {
		&self.strings[pos]
	}

	/// Read a [`Value`] from the [`Value`] storage
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
			OP_BINARY_NOT => Self::simple_instruction("OP_BINARY_NOT", offset),
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
			OP_POP => Self::simple_instruction("OP_POP", offset),
			OP_PRINT => Self::simple_instruction("OP_PRINT", offset),
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
					values::VAL_BOOL => {
						std::println!("{name:16} {pos:3} {}", value.as_bool().expect("snh"));
					}
					values::VAL_DOUBLE => {
						std::println!("{name:16} {pos:3} {}", value.as_double().expect("snh"));
					}
					values::VAL_INT => {
						std::println!("{name:16} {pos:3} {}", value.as_integer().expect("snh"));
					}
					values::VAL_STR => {
						std::println!(
							"{name:16} {pos:3} {}",
							self.strings[value.as_string_pos().expect("snh")]
						);
					}
					values::VAL_NIL => {
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
