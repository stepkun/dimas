// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Bytecode implementation for `DiMAS` scripting

#[doc(hidden)]
extern crate std;

use alloc::{borrow::ToOwned, vec::Vec};

#[allow(clippy::wildcard_imports)]
use crate::scripting::execution::opcodes::*;
use crate::scripting::execution::values::{HexNumbers, HexValue, Numbers, Value};

/// A chunk of bytecode
#[derive(Default)]
pub struct Chunk {
	/// the code
	code: Vec<u8>,
	/// corresponding storage for the line number
	lines: Vec<i16>,
	/// storage for f64 values
	numbers: Numbers,
	/// storage for i64/hex values
	hex_numbers: HexNumbers,
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
		let pos = self.numbers.write(value);
		pos as u8
	}

	/// Read a f64 number from the number storage
	#[must_use]
	pub fn read_constant(&self, pos: u8) -> Value {
		let offset = usize::from(pos);
		self.numbers.read(offset)
	}

	/// Add a i64 number to the hex number storage returning its position in the storage
	#[allow(clippy::cast_possible_truncation)]
	pub fn add_hex_constant(&mut self, value: HexValue) -> u8 {
		let pos = self.hex_numbers.write(value);
		pos as u8
	}

	/// Read a i64 number from the number storage
	#[must_use]
	pub fn read_hex_constant(&self, pos: u8) -> HexValue {
		let offset = usize::from(pos);
		self.hex_numbers.read(offset)
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
			OP_RETURN => Self::simple_instruction("OP_RETURN", offset),
			OP_ADD => Self::simple_instruction("OP_ADD", offset),
			OP_SUBTRACT => Self::simple_instruction("OP_SUBTRACT", offset),
			OP_MULTIPLY => Self::simple_instruction("OP_MULTIPLY", offset),
			OP_DIVIDE => Self::simple_instruction("OP_DIVIDE", offset),
			OP_NEGATE => Self::simple_instruction("OP_NEGATE", offset),
			OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
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
			Some(value_pos) => {
				let pos = usize::from(value_pos.to_owned());
				std::println!("{name:16} {value_pos:3} {}", self.numbers.read(pos));
			}
			None => todo!(),
		}
		offset + 2
	}
}
