// Copyright © 2025 Stephan Kunz

//! Value implementations for `DiMAS` scripting
//! `Numbers` are always f64 and `HexNumbers` are always i64

use core::fmt::Display;

use super::error::Error;

/// Enum for working with different types of [`Value`]s
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum ValueType {
	/// @TODO:
	Nil,
	/// @TODO:
	Bool,
	/// @TODO:
	Double,
	/// @TODO:
	Int,
	/// @TODO:
	Str,
}

/// The inner part of the `Value` type as union of available types
#[derive(Clone, Copy)]
union InnerValue {
	boolean: bool,
	integer: i64,
	double: f64,
	string_pos: usize, // is the position in the String storage
}

/// Definition of the `Value` type
#[derive(Clone, Copy)]
pub struct Value {
	kind: ValueType,
	value: InnerValue,
}

impl core::fmt::Debug for Value {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.kind {
			ValueType::Bool => f
				.debug_struct("Value")
				.field("kind", &"bool")
				.field("value", &self.as_bool())
				.finish(),
			ValueType::Double => f
				.debug_struct("Value")
				.field("kind", &"double")
				.field("value", &self.as_double())
				.finish(),
			ValueType::Int => f
				.debug_struct("Value")
				.field("kind", &"int")
				.field("value", &self.as_integer())
				.finish(),
			ValueType::Nil => f
				.debug_struct("Value")
				.field("kind", &"nil")
				.field("value", &"nil")
				.finish(),
			ValueType::Str => f
				.debug_struct("Value")
				.field("kind", &"str_pos")
				.field("value", &self.as_string_pos())
				.finish(),
		}
	}
}

impl Default for Value {
	fn default() -> Self {
		Self {
			kind: ValueType::Nil,
			value: InnerValue { integer: 0 },
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.kind {
			ValueType::Bool => write!(f, "{}", self.as_bool().expect("snh")),
			ValueType::Double => write!(f, "{}", self.as_double().expect("snh")),
			ValueType::Int => write!(f, "{}", self.as_integer().expect("snh")),
			ValueType::Str => write!(f, "pos: {}", self.as_string_pos().expect("snh")),
			ValueType::Nil => write!(f, "nil"),
		}
	}
}

impl Value {
	/// @TODO:
	#[must_use]
	pub const fn kind(&self) -> ValueType {
		self.kind
	}

	/// @TODO:
	#[must_use]
	pub const fn nil() -> Self {
		Self {
			kind: ValueType::Nil,
			value: InnerValue { integer: 0 },
		}
	}

	/// @TODO:
	pub const fn make_nil(&mut self) {
		self.kind = ValueType::Nil;
		self.value.integer = 0;
	}

	/// @TODO:
	#[must_use]
	pub fn is_nil(&self) -> bool {
		self.kind == ValueType::Nil
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub fn as_bool(&self) -> Result<bool, Error> {
		if self.kind == ValueType::Bool {
			Ok(unsafe { self.value.boolean })
		} else {
			Err(Error::NoBoolean)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_bool(boolean: bool) -> Self {
		Self {
			kind: ValueType::Bool,
			value: InnerValue { boolean },
		}
	}

	/// @TODO:
	#[must_use]
	pub fn is_bool(&self) -> bool {
		self.kind == ValueType::Bool
	}

	/// @TODO:
	pub const fn make_bool(&mut self, boolean: bool) {
		self.kind = ValueType::Bool;
		self.value.boolean = boolean;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub fn as_double(&self) -> Result<f64, Error> {
		if self.kind == ValueType::Double {
			Ok(unsafe { self.value.double })
		} else {
			Err(Error::NoDouble)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_double(double: f64) -> Self {
		Self {
			kind: ValueType::Double,
			value: InnerValue { double },
		}
	}

	/// @TODO:
	#[must_use]
	pub fn is_double(&self) -> bool {
		self.kind == ValueType::Double
	}

	/// @TODO:
	pub const fn make_double(&mut self, double: f64) {
		self.kind = ValueType::Double;
		self.value.double = double;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub fn as_integer(&self) -> Result<i64, Error> {
		if self.kind == ValueType::Int {
			Ok(unsafe { self.value.integer })
		} else {
			Err(Error::NoInteger)
		}
	}

	/// @TODO:
	/// # Errors
	#[must_use]
	pub const fn from_integer(integer: i64) -> Self {
		Self {
			kind: ValueType::Int,
			value: InnerValue { integer },
		}
	}

	/// @TODO:
	#[must_use]
	pub fn is_integer(&self) -> bool {
		self.kind == ValueType::Int
	}

	/// @TODO:
	pub const fn make_integer(&mut self, integer: i64) {
		self.kind = ValueType::Int;
		self.value.integer = integer;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub fn as_string_pos(&self) -> Result<usize, Error> {
		if self.kind == ValueType::Str {
			Ok(unsafe { self.value.string_pos })
		} else {
			Err(Error::NoString)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_string_pos(string_pos: usize) -> Self {
		Self {
			kind: ValueType::Str,
			value: InnerValue { string_pos },
		}
	}

	/// @TODO:
	#[must_use]
	pub fn is_string_pos(&self) -> bool {
		self.kind == ValueType::Str
	}

	/// @TODO:
	pub const fn make_string_pos(&mut self, string_pos: usize) {
		self.kind = ValueType::Str;
		self.value.string_pos = string_pos;
	}
}
