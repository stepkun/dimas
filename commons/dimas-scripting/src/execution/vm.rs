// Copyright © 2025 Stephan Kunz

//! Virtual machine for `DiMAS` scripting

extern crate std;

use alloc::borrow::ToOwned;
// region:		--- modules
use alloc::string::ToString;
use dimas_core::value::Value;

use crate::Environment;

use super::op_code::OpCode;
use super::{Chunk, error::Error};
// endregion:	--- modules

/// Stack size is fixed to avoid cache misses, which drastically reduce performance.
/// For the intended purpose (short inline scripting) this size should be enough.
const STACK_SIZE: usize = 8;

// region:		--- VM
/// A stack based Virtual Machine.
///
/// The stack size is limited to avoid cache misses, which drastically reduce performance.
/// For the intended purpose (short inline scripting) this size should be enough.
pub struct VM {
	/// The `InstructionPointer` (sometimes called `ProgramCounter`)
	ip: usize,
	/// Stack for values
	stack: [Value; STACK_SIZE],
	/// Pointer to the next free stack place
	stack_top: usize,
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack: [const { Value::nil() }; STACK_SIZE],
			stack_top: 0,
		}
	}
}

impl VM {
	fn reset(&mut self) {
		self.ip = 0;
		self.stack = [const { Value::nil() }; STACK_SIZE];
		self.stack_top = 0;
	}

	const fn peek(&self, distance: usize) -> &Value {
		&self.stack[self.stack_top - distance - 1]
	}

	fn push(&mut self, value: Value) -> Result<(), Error> {
		if self.stack_top == u8::MAX as usize {
			return Err(Error::StackOverflow);
		}
		self.stack[self.stack_top] = value;
		self.stack_top += 1;
		Ok(())
	}

	fn pop(&mut self) -> Value {
		self.stack_top -= 1;
		self.stack[self.stack_top].clone()
	}

	fn read_jmp_address(&mut self, chunk: &Chunk) -> usize {
		let byte1 = chunk.code()[self.ip];
		self.ip += 1;
		let byte2 = chunk.code()[self.ip];
		self.ip += 1;
		((byte1 as usize) << 8) + byte2 as usize
	}

	#[allow(clippy::cast_precision_loss)]
	fn arithmetic_operator(&mut self, operator: &OpCode) -> Result<(), Error> {
		let b_val = self.pop();
		let a_val = self.pop();
		match (&a_val, &b_val) {
			(Value::Float64(a), Value::Float64(b)) => {
				let res = match operator {
					OpCode::Add => a + b,
					OpCode::Subtract => a - b,
					OpCode::Multiply => a * b,
					OpCode::Divide => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Float64(res))
			}
			(Value::Float64(a), Value::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + (*b as f64),
					OpCode::Subtract => a - (*b as f64),
					OpCode::Multiply => a * (*b as f64),
					OpCode::Divide => a / (*b as f64),
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Float64(res))
			}
			(Value::Int64(a), Value::Float64(b)) => {
				let res = match operator {
					OpCode::Add => (*a as f64) + b,
					OpCode::Subtract => (*a as f64) - b,
					OpCode::Multiply => (*a as f64) * b,
					OpCode::Divide => (*a as f64) / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Float64(res))
			}
			(Value::Int64(a), Value::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + b,
					OpCode::Subtract => a - b,
					OpCode::Multiply => a * b,
					OpCode::Divide => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Int64(res))
			}
			(Value::String(a), _) => {
				let res = match operator {
					OpCode::Add => a.to_owned() + &b_val.to_string(),
					_ => return Err(Error::OnlyAdd),
				};
				self.push(Value::String(res))
			}
			(_, Value::String(b)) => {
				let res = match operator {
					OpCode::Add => a_val.to_string() + b,
					_ => return Err(Error::OnlyAdd),
				};
				self.push(Value::String(res))
			}
			(Value::Nil(), _) | (_, Value::Nil()) => Err(Error::NilValue),
			(Value::Boolean(_), _) | (_, Value::Boolean(_)) => Err(Error::BoolNoArithmetic),
		}
	}

	fn bitwise_operator(&mut self, operator: &OpCode) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		match (a_val, b_val) {
			(Value::Int64(a), Value::Int64(b)) => {
				let res = match operator {
					OpCode::BitwiseAnd => a & b,
					OpCode::BitwiseOr => a | b,
					OpCode::BitwiseXor => a ^ b,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val = Value::Int64(res);
				self.push(a_val)
			}
			_ => Err(Error::NoInteger),
		}
	}

	#[allow(clippy::cast_precision_loss)]
	fn comparison_operator(&mut self, operator: &OpCode) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		let res = match (a_val, b_val) {
			(Value::Int64(a), Value::Int64(b)) => match operator {
				OpCode::Greater => a > b,
				OpCode::Less => a < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			(Value::Int64(a), Value::Float64(b)) => match operator {
				OpCode::Greater => (a as f64) > b,
				OpCode::Less => (a as f64) < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			(Value::Float64(a), Value::Int64(b)) => match operator {
				OpCode::Greater => a > (b as f64),
				OpCode::Less => a < (b as f64),
				_ => return Err(Error::Unreachable(line!())),
			},
			(Value::Float64(a), Value::Float64(b)) => match operator {
				OpCode::Greater => a > b,
				OpCode::Less => a < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			_ => return Err(Error::NoComparison),
		};
		a_val = Value::Boolean(res);
		self.push(a_val)
	}

	fn constant(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let constant = chunk.read_constant(pos);
		self.ip += 1;
		self.push(constant)
	}

	#[allow(clippy::cast_precision_loss)]
	fn equal(&mut self) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		let res = match (a_val, b_val) {
			(Value::Boolean(a), Value::Boolean(b)) => a == b,
			(Value::Float64(a), Value::Float64(b)) => {
				let delta = f64::abs(a - b);
				delta <= 0.000_000_000_000_002
			}
			(Value::Float64(a), Value::Int64(b)) => {
				let delta = f64::abs(a - (b as f64));
				delta <= 0.000_000_000_000_002
			}
			(Value::Int64(a), Value::Float64(b)) => {
				let delta = f64::abs((a as f64) - b);
				delta <= 0.000_000_000_000_002
			}
			(Value::Int64(a), Value::Int64(b)) => a == b,
			(Value::String(a), Value::String(b)) => a == b,
			(Value::Nil(), Value::Nil()) => true,
			_ => false,
		};
		a_val = Value::Boolean(res);
		self.push(a_val)
	}

	fn negate(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			Value::Int64(v) => Value::Int64(-v),
			Value::Float64(v) => Value::Float64(-v),
			_ => return Err(Error::NoNumber),
		};
		self.push(res)
	}

	fn bitwise_not(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			Value::Int64(v) => Value::Int64(!v),
			_ => return Err(Error::NoNumber),
		};
		self.push(res)
	}

	fn not(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			Value::Boolean(b) => Value::Boolean(!b),
			Value::Nil() => Value::Boolean(true),
			_ => Value::Boolean(false),
		};
		self.push(res)
	}

	#[cfg(feature = "std")]
	fn print(&mut self, stdout: &mut impl std::io::Write) {
		if self.stack_top > 0 {
			let value = self.pop();
			let _ = std::writeln!(stdout, "{value}");
		} else {
			let _ = std::writeln!(stdout, "no result");
		}
	}

	fn define_global(&mut self, chunk: &Chunk, globals: &dyn Environment) {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		let value_val = self.pop();
		//let name = chunk.get_string(name_val.as_string_pos()?);
		globals.define_env(&name_val.to_string(), value_val);
	}

	fn get_global(&mut self, chunk: &Chunk, globals: &dyn Environment) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		// let name = chunk.get_string(name_val.as_string_pos()?);
		let val = globals.get_env(&name_val.to_string())?;
		self.push(val)?;
		Ok(())
	}

	fn set_global(&mut self, chunk: &Chunk, globals: &dyn Environment) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		// let name = chunk.get_string(name_val.as_string_pos()?);
		let value_val = self.pop();
		globals.set_env(&name_val.to_string(), value_val)
	}

	/// Execute a [`Chunk`] with the virtual machine,
	/// Returns the topmost stack [`Value`] if there is one, otherwise [`Value::nil()`].
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(
		&mut self,
		chunk: &Chunk,
		globals: &dyn Environment,
		#[cfg(feature = "std")] stdout: &mut impl std::io::Write,
	) -> Result<Value, Error> {
		self.reset();
		// ignore empty chunks
		if chunk.code().is_empty() {
			return Ok(Value::nil());
		}

		loop {
			//std::dbg!(self.ip);
			let instruction: OpCode = chunk.code()[self.ip].into();
			self.ip += 1;
			match instruction {
				OpCode::Add | OpCode::Divide | OpCode::Multiply | OpCode::Subtract => {
					self.arithmetic_operator(&instruction)?;
				}
				OpCode::BitwiseAnd | OpCode::BitwiseOr | OpCode::BitwiseXor => {
					self.bitwise_operator(&instruction)?;
				}
				OpCode::BitwiseNot => self.bitwise_not()?,
				OpCode::Constant => self.constant(chunk)?,
				OpCode::DefineExternal => self.define_global(chunk, globals),
				OpCode::Equal => self.equal()?,
				OpCode::False => self.push(Value::Boolean(false))?,
				OpCode::GetExternal => self.get_global(chunk, globals)?,
				OpCode::Greater => self.comparison_operator(&instruction)?,
				OpCode::Jmp => {
					let target = self.read_jmp_address(chunk);
					self.ip = target;
				}
				OpCode::JmpIfFalse => {
					let target = self.read_jmp_address(chunk);
					if !self.peek(0).as_bool()? {
						self.ip = target;
					}
				}
				OpCode::JmpIfTrue => {
					let target = self.read_jmp_address(chunk);
					if self.peek(0).as_bool()? {
						self.ip = target;
					}
				}
				OpCode::Less => self.comparison_operator(&instruction)?,
				OpCode::Negate => self.negate()?,
				OpCode::Nil => self.push(Value::nil())?,
				OpCode::Not => self.not()?,
				OpCode::Pop => {
					self.pop();
				}
				#[cfg(feature = "std")]
				OpCode::Print => self.print(stdout),
				OpCode::Return => {
					let val = if self.stack_top > 0 {
						self.pop()
					} else {
						Value::nil()
					};
					//chunk.restore_state();
					return Ok(val);
				}
				OpCode::SetExternal => self.set_global(chunk, globals)?,
				OpCode::True => self.push(Value::Boolean(true))?,
				_ => {
					return Err(Error::UnknownOpCode);
				}
			}
		}
	}
}
// endregion:	--- VM
