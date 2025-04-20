// Copyright © 2025 Stephan Kunz

//! Virtual machine for `DiMAS` scripting

extern crate std;

// region:		--- modules
use alloc::{borrow::ToOwned, string::ToString};

use crate::Environment;

use super::{Chunk, ScriptingValue, error::Error, op_code::OpCode};
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
	stack: [ScriptingValue; STACK_SIZE],
	/// Pointer to the next free stack place
	stack_top: usize,
}

impl core::fmt::Debug for VM {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("VM")
			.field("ip", &self.ip)
			.field("stack", &self.stack)
			.field("stack_top", &self.stack_top)
			.finish()
	}
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack: [const { ScriptingValue::nil() }; STACK_SIZE],
			stack_top: 0,
		}
	}
}

impl VM {
	fn reset(&mut self) {
		self.ip = 0;
		self.stack = [const { ScriptingValue::nil() }; STACK_SIZE];
		self.stack_top = 0;
	}

	const fn peek(&self, distance: usize) -> &ScriptingValue {
		&self.stack[self.stack_top - distance - 1]
	}

	fn push(&mut self, value: ScriptingValue) -> Result<(), Error> {
		if self.stack_top == u8::MAX as usize {
			return Err(Error::StackOverflow);
		}
		self.stack[self.stack_top] = value;
		self.stack_top += 1;
		Ok(())
	}

	fn pop(&mut self) -> ScriptingValue {
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
			(ScriptingValue::Float64(a), ScriptingValue::Float64(b)) => {
				let res = match operator {
					OpCode::Add => a + b,
					OpCode::Subtract => a - b,
					OpCode::Multiply => a * b,
					OpCode::Divide => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(ScriptingValue::Float64(res))
			}
			(ScriptingValue::Float64(a), ScriptingValue::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + (*b as f64),
					OpCode::Subtract => a - (*b as f64),
					OpCode::Multiply => a * (*b as f64),
					OpCode::Divide => a / (*b as f64),
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(ScriptingValue::Float64(res))
			}
			(ScriptingValue::Int64(a), ScriptingValue::Float64(b)) => {
				let res = match operator {
					OpCode::Add => (*a as f64) + b,
					OpCode::Subtract => (*a as f64) - b,
					OpCode::Multiply => (*a as f64) * b,
					OpCode::Divide => (*a as f64) / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(ScriptingValue::Float64(res))
			}
			(ScriptingValue::Int64(a), ScriptingValue::Int64(b)) => {
				let res = match operator {
					OpCode::Add => a + b,
					OpCode::Subtract => a - b,
					OpCode::Multiply => a * b,
					OpCode::Divide => a / b,
					_ => return Err(Error::Unreachable(line!())),
				};
				self.push(ScriptingValue::Int64(res))
			}
			(ScriptingValue::String(a), _) => {
				let res = match operator {
					OpCode::Add => a.to_owned() + &b_val.to_string(),
					_ => return Err(Error::OnlyAdd),
				};
				self.push(ScriptingValue::String(res))
			}
			(_, ScriptingValue::String(b)) => {
				let res = match operator {
					OpCode::Add => a_val.to_string() + b,
					_ => return Err(Error::OnlyAdd),
				};
				self.push(ScriptingValue::String(res))
			}
			(ScriptingValue::Nil(), _) | (_, ScriptingValue::Nil()) => Err(Error::NilValue),
			(ScriptingValue::Boolean(_), _) | (_, ScriptingValue::Boolean(_)) => {
				Err(Error::BoolNoArithmetic)
			}
		}
	}

	fn bitwise_operator(&mut self, operator: &OpCode) -> Result<(), Error> {
		let b_val = self.pop();
		let mut a_val = self.pop();
		match (a_val, b_val) {
			(ScriptingValue::Int64(a), ScriptingValue::Int64(b)) => {
				let res = match operator {
					OpCode::BitwiseAnd => a & b,
					OpCode::BitwiseOr => a | b,
					OpCode::BitwiseXor => a ^ b,
					_ => return Err(Error::Unreachable(line!())),
				};
				a_val = ScriptingValue::Int64(res);
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
			(ScriptingValue::Int64(a), ScriptingValue::Int64(b)) => match operator {
				OpCode::Greater => a > b,
				OpCode::Less => a < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			(ScriptingValue::Int64(a), ScriptingValue::Float64(b)) => match operator {
				OpCode::Greater => (a as f64) > b,
				OpCode::Less => (a as f64) < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			(ScriptingValue::Float64(a), ScriptingValue::Int64(b)) => match operator {
				OpCode::Greater => a > (b as f64),
				OpCode::Less => a < (b as f64),
				_ => return Err(Error::Unreachable(line!())),
			},
			(ScriptingValue::Float64(a), ScriptingValue::Float64(b)) => match operator {
				OpCode::Greater => a > b,
				OpCode::Less => a < b,
				_ => return Err(Error::Unreachable(line!())),
			},
			_ => return Err(Error::NoComparison),
		};
		a_val = ScriptingValue::Boolean(res);
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
			(ScriptingValue::Boolean(a), ScriptingValue::Boolean(b)) => a == b,
			(ScriptingValue::Float64(a), ScriptingValue::Float64(b)) => {
				let delta = f64::abs(a - b);
				delta <= 0.000_000_000_000_002
			}
			(ScriptingValue::Float64(a), ScriptingValue::Int64(b)) => {
				let delta = f64::abs(a - (b as f64));
				delta <= 0.000_000_000_000_002
			}
			(ScriptingValue::Int64(a), ScriptingValue::Float64(b)) => {
				let delta = f64::abs((a as f64) - b);
				delta <= 0.000_000_000_000_002
			}
			(ScriptingValue::Int64(a), ScriptingValue::Int64(b)) => a == b,
			(ScriptingValue::String(a), ScriptingValue::String(b)) => a == b,
			(ScriptingValue::Nil(), ScriptingValue::Nil()) => true,
			_ => false,
		};
		a_val = ScriptingValue::Boolean(res);
		self.push(a_val)
	}

	fn negate(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			ScriptingValue::Int64(v) => ScriptingValue::Int64(-v),
			ScriptingValue::Float64(v) => ScriptingValue::Float64(-v),
			_ => return Err(Error::NoNumber),
		};
		self.push(res)
	}

	fn bitwise_not(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			ScriptingValue::Int64(v) => ScriptingValue::Int64(!v),
			_ => return Err(Error::NoNumber),
		};
		self.push(res)
	}

	fn not(&mut self) -> Result<(), Error> {
		let val = self.pop();
		let res = match val {
			ScriptingValue::Boolean(b) => ScriptingValue::Boolean(!b),
			ScriptingValue::Nil() => ScriptingValue::Boolean(true),
			_ => ScriptingValue::Boolean(false),
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

	fn define_global(&mut self, chunk: &Chunk, globals: &dyn Environment) -> Result<(), Error> {
		let pos = chunk.code()[self.ip];
		let name_val = chunk.read_constant(pos);
		self.ip += 1;
		let value_val = self.pop();
		//let name = chunk.get_string(name_val.as_string_pos()?);
		globals.define_env(&name_val.to_string(), value_val)
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
	/// Returns the topmost stack [`ScriptingValue`] if there is one, otherwise [`ScriptingValue::nil()`].
	/// # Errors
	/// - unknown `OpCode`
	pub fn run(
		&mut self,
		chunk: &Chunk,
		globals: &dyn Environment,
		#[cfg(feature = "std")] stdout: &mut impl std::io::Write,
	) -> Result<ScriptingValue, Error> {
		self.reset();
		// ignore empty chunks
		if chunk.code().is_empty() {
			return Ok(ScriptingValue::nil());
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
				OpCode::DefineExternal => self.define_global(chunk, globals)?,
				OpCode::Equal => self.equal()?,
				OpCode::False => self.push(ScriptingValue::Boolean(false))?,
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
				OpCode::Nil => self.push(ScriptingValue::nil())?,
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
						ScriptingValue::nil()
					};
					//chunk.restore_state();
					return Ok(val);
				}
				OpCode::SetExternal => self.set_global(chunk, globals)?,
				OpCode::True => self.push(ScriptingValue::Boolean(true))?,
				_ => {
					return Err(Error::UnknownOpCode);
				}
			}
		}
	}
}
// endregion:	--- VM
