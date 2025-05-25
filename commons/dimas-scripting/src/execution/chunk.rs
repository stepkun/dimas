// Copyright © 2025 Stephan Kunz

//! Bytecode implementation for `DiMAS` scripting

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{borrow::ToOwned, vec::Vec};

use crate::execution::op_code::OpCode;

use super::{Error, ScriptingValue};
// endregion:   --- modules

/// A chunk of bytecode
#[derive(Default)]
pub struct Chunk {
	/// the code
	code: Vec<u8>,
	/// corresponding storage for the line number
	lines: Vec<usize>,
	/// storage for Values
	values: Vec<ScriptingValue>,
}

impl Chunk {
	/// Access code
	#[must_use]
	pub const fn code(&self) -> &Vec<u8> {
		&self.code
	}

	/// Finalizes the [`Chunk`] by shrinking al [`Vec`]'s
	pub(crate) fn finalize(&mut self) {
		self.code.shrink_to_fit();
		self.lines.shrink_to_fit();
		self.values.shrink_to_fit();
	}

	/// Add a byte to the chunk
	pub fn write(&mut self, byte: u8, line: usize) {
		self.code.push(byte);
		self.lines.push(line);
	}

	/// Patch a byte in the chunk
	pub fn patch(&mut self, byte: u8, pos: usize) {
		self.code[pos] = byte;
	}

	/// Add a Value to the Value storage returning its position in the storage
	/// # Errors
	#[allow(clippy::cast_possible_truncation)]
	pub fn add_constant(&mut self, value: ScriptingValue) -> Result<u8, Error> {
		if self.values.len() < u8::MAX as usize {
			self.values.push(value);
			let pos = self.values.len() - 1;
			Ok(pos as u8)
		} else {
			Err(Error::StackOverflow)
		}
	}

	/*
		/// Add a String to the String storage returning its position in the storage
		pub fn add_string(&mut self, string: String) -> usize {
			self.strings.push(string);
			self.strings.len() - 1
		}

		/// Add a String to the Value storage returning its position in the storage
		/// # Errors
		pub fn add_string_constant(&mut self, string: String) -> Result<u8, Error> {
			let offset = self.add_string(string);
			let value = ScriptingValue::from_string_pos(offset);
			self.add_constant(value)
		}

		/// Get the reference to stored [`String`]
		#[must_use]
		pub fn get_string(&self, pos: usize) -> &String {
			&self.strings[pos]
		}
	*/
	/// Read a [`ScriptingValue`] from the [`ScriptingValue`] storage
	#[must_use]
	pub fn read_constant(&self, pos: u8) -> ScriptingValue {
		let offset = usize::from(pos);
		self.values
			.get(offset)
			.map_or_else(|| todo!("pos: {}", pos), ToOwned::to_owned)
	}

	/// Disassemble chunk
	#[cfg(feature = "std")]
	pub fn disassemble(&self, name: &str) {
		let mut offset = 0usize;
		std::println!("== {name} ==");
		while offset < self.code.len() {
			offset = self.disassemble_instruction(offset);
		}
	}

	/// Disassemble a single instruction
	#[cfg(feature = "std")]
	fn disassemble_instruction(&self, offset: usize) -> usize {
		std::print!("{offset:04} ");
		if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
			std::print!("   | ");
		} else {
			std::print!("{:4} ", self.lines[offset]);
		}
		let instruction: OpCode = self.code[offset].into();
		match instruction {
			OpCode::Add => Self::simple_instruction("OP_ADD", offset),
			OpCode::BitwiseAnd => Self::simple_instruction("OP_BITWISE_AND", offset),
			OpCode::BitwiseNot => Self::simple_instruction("OP_BITWISE_NOT", offset),
			OpCode::BitwiseOr => Self::simple_instruction("OP_BITWISE_OR", offset),
			OpCode::BitwiseXor => Self::simple_instruction("OP_BITWISE_XOR", offset),
			OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
			OpCode::DefineExternal => self.constant_instruction("OP_DEFINE_GLOBAL", offset),
			OpCode::Divide => Self::simple_instruction("OP_DIVIDE", offset),
			OpCode::Equal => Self::simple_instruction("OP_EQUAL", offset),
			OpCode::False => Self::simple_instruction("OP_FALSE", offset),
			OpCode::GetExternal => self.constant_instruction("OP_GET_GLOBAL", offset),
			OpCode::Greater => Self::simple_instruction("OP_GREATER", offset),
			OpCode::Jmp => self.jump_instruction("OP_JMP", offset),
			OpCode::JmpIfFalse => self.jump_instruction("OP_JMP_IF_FALSE", offset),
			OpCode::JmpIfTrue => self.jump_instruction("OP_JMP_IF_TRUE", offset),
			OpCode::Less => Self::simple_instruction("OP_LESS", offset),
			OpCode::Multiply => Self::simple_instruction("OP_MULTIPLY", offset),
			OpCode::Negate => Self::simple_instruction("OP_NEGATE", offset),
			OpCode::Nil => Self::simple_instruction("OP_NIL", offset),
			OpCode::None => Self::simple_instruction("OP_NONE", offset),
			OpCode::Not => Self::simple_instruction("OP_NOT", offset),
			OpCode::Pop => Self::simple_instruction("OP_POP", offset),
			OpCode::Print => Self::simple_instruction("OP_PRINT", offset),
			OpCode::Return => Self::simple_instruction("OP_RETURN", offset),
			OpCode::SetExternal => self.constant_instruction("OP_SET_GLOBAL", offset),
			OpCode::Subtract => Self::simple_instruction("OP_SUBTRACT", offset),
			OpCode::True => Self::simple_instruction("OP_TRUE", offset),
		}
	}

	/// single byte instruction
	#[cfg(feature = "std")]
	fn simple_instruction(name: &str, offset: usize) -> usize {
		std::println!("{name:16}");
		offset + 1
	}

	/// constant instruction
	#[cfg(feature = "std")]
	fn constant_instruction(&self, name: &str, offset: usize) -> usize {
		match self.code.get(offset + 1) {
			Some(pos) => {
				let value = self.read_constant(pos.to_owned());
				match value {
					ScriptingValue::Nil() => std::println!("{name:16} {pos:3} 'NIL'"),
					ScriptingValue::Boolean(b) => std::println!("{name:16} {pos:3} {b}"),
					ScriptingValue::Float64(f) => std::println!("{name:16} {pos:3} {f}"),
					ScriptingValue::Int64(i) => std::println!("{name:16} {pos:3} {i}"),
					ScriptingValue::String(s) => std::println!("{name:16} {pos:3} {s}"),
				}
			}
			None => todo!(),
		}
		offset + 2
	}

	/// constant instruction
	#[cfg(feature = "std")]
	fn jump_instruction(&self, name: &str, offset: usize) -> usize {
		let target = (usize::from(self.code.get(offset + 1).expect("snh").to_owned()) << 8)
			+ usize::from(self.code.get(offset + 2).expect("snh").to_owned());

		std::println!("{name:16} {offset:05} {target:05}");
		offset + 3
	}
}
