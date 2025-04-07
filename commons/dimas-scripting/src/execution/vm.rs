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

/// Stack size is fixed
const STACK_MAX: usize = 256;

// region:		--- VM
/// A Virtual Machine
pub struct VM<'a> {
	/// The `InstructionPointer` (sometimes called `ProgramCounter`)
	ip: usize,
	/// Stack for values
	stack: [Value; STACK_MAX],
	/// Pointer to the next free stack place
	stack_top: usize,
	/// Reference to a storage for truly `global` variables, which are used also available outside the [`VM`].
	/// The storage has to provide getter and setter methods using interior mutability.
	globals: &'a dyn Environment,
}

impl<'a> VM<'a> {
	/// Create a [`VM`] with an external Environment
	pub fn new(environment: &'a dyn Environment) -> Self {
		Self {
			ip: 0,
			stack: [const { Value::nil() }; STACK_MAX],
			stack_top: 0,
			globals: environment,
		}
	}

	fn reset(&mut self) {
		self.ip = 0;
		self.stack = [const { Value::nil() }; STACK_MAX];
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
			},
			(Value::Float64(a), Value::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + (*b as f64),
					OpCode::Subtract => a - (*b as f64),
					OpCode::Multiply => a * (*b as f64),
					OpCode::Divide => a / (*b as f64),
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Float64(res))
			},
			(Value::Int64(a), Value::Float64(b)) => {
				let res = match operator {
					OpCode::Add => (*a as f64) + b,
					OpCode::Subtract => (*a as f64) - b,
					OpCode::Multiply => (*a as f64) * b,
					OpCode::Divide => (*a as f64) / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Float64(res))
			},
			(Value::Int64(a), Value::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + b,
					OpCode::Subtract => a - b,
					OpCode::Multiply => a * b,
					OpCode::Divide => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(Value::Int64(res))
			},
			(Value::String(a), _) => {
				let res = match operator {
					OpCode::Add => a.to_owned() + &b_val.to_string(),
					_ => return Err(Error::OnlyAdd),
				};
				self.push(Value::String(res))
			},
			(_, Value::String(b)) => {
				let res = match operator {
					OpCode::Add => a_val.to_string() + b,
					_ => return Err(Error::OnlyAdd),
				};
				self.push(Value::String(res))
			},
			(Value::Nil(), _) | (_, Value::Nil()) => Err(Error::NilValue),
			(Value::Boolean(_), _) | (_, Value::Boolean(_)) => Err(Error::BoolNoArithmetic),
			(Value::Dynamic(_), _) => todo!(),
			(_, Value::Dynamic(_)) => todo!(),
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

	fn equal(&mut self) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		let res = match (a_val, b_val) {
			(Value::Boolean(a), Value::Boolean(b)) => a == b,
			(Value::Int64(a), Value::Int64(b)) => a == b,
			(Value::Float64(a), Value::Float64(b)) => {
				let delta = f64::abs(a - b);
				delta <= 0.000_000_000_000_002
			},
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

	fn define_global(&mut self, chunk: &Chunk) {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		let value_val = self.pop();
		//let name = chunk.get_string(name_val.as_string_pos()?);
		self.globals
			.define_env(&name_val.to_string(), value_val);
	}

	fn get_global(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		// let name = chunk.get_string(name_val.as_string_pos()?);
		let val = self.globals.get_env(&name_val.to_string())?;
		self.push(val)?;
		Ok(())
	}

	fn set_global(&mut self, chunk: &Chunk) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		// let name = chunk.get_string(name_val.as_string_pos()?);
		let value_val = self.pop();
		self.globals
			.set_env(&name_val.to_string(), value_val)
	}

	/// Execute a [`Chunk`] with the virtual machine,
	/// Returns the topmost stack [`Value`] if there is one, otherwise [`Value::nil()`].
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(
		&mut self,
		chunk: &mut Chunk,
		#[cfg(feature = "std")] stdout: &mut impl std::io::Write,
	) -> Result<Value, Error> {
		self.reset();
		chunk.save_state();
		// ignore empty chunks
		if chunk.code().is_empty() {
			chunk.restore_state();
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
				OpCode::DefineExternal => self.define_global(chunk),
				OpCode::Equal => self.equal()?,
				OpCode::False => self.push(Value::Boolean(false))?,
				OpCode::GetExternal => self.get_global(chunk)?,
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
					chunk.restore_state();
					return Ok(val);
				}
				OpCode::SetExternal => self.set_global(chunk)?,
				OpCode::True => self.push(Value::Boolean(true))?,
				_ => {
					chunk.restore_state();
					return Err(Error::UnknownOpCode);
				}
			}
		}
	}
}
// endregion:	--- VM
