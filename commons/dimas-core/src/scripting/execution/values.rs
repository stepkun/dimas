// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::redundant_closure_for_method_calls)]

//! Value implementations for `DiMAS` scripting
//! `Numbers` are always f64 and `HexNumbers` are always i64

use core::{fmt::Display, mem::ManuallyDrop};

use alloc::{borrow::ToOwned, string::String, vec::Vec};

use super::{Chunk, VM, error::Error};

/// Constants for working with different types of [`Value`]s
pub(crate) const VAL_NIL: i8 = 0;
pub(crate) const VAL_BOOL: i8 = VAL_NIL + 1;
pub(crate) const VAL_DOUBLE: i8 = VAL_BOOL + 1;
pub(crate) const VAL_HEX: i8 = VAL_DOUBLE + 1;
pub(crate) const VAL_INT: i8 = VAL_HEX;
pub(crate) const VAL_STR: i8 = VAL_INT + 1;

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
	kind: i8,
	value: InnerValue,
}

impl Default for Value {
	fn default() -> Self {
		Self {
			kind: VAL_NIL,
			value: InnerValue { integer: 0 },
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.kind {
			VAL_BOOL => write!(f, "{}", self.as_bool().expect("snh")),
			VAL_DOUBLE => write!(f, "{}", self.as_double().expect("snh")),
			VAL_INT => write!(f, "{}", self.as_integer().expect("snh")),
			VAL_STR => write!(f, "pos: {}", self.as_string_pos().expect("snh")),
			_ => write!(f, "nil"),
		}
	}
}

impl Value {
	/// @TODO:
	#[must_use]
	pub const fn kind(&self) -> i8 {
		self.kind
	}

	/// @TODO:
	#[must_use]
	pub const fn nil() -> Self {
		Self {
			kind: VAL_NIL,
			value: InnerValue { integer: 0 },
		}
	}

	/// @TODO:
	pub const fn to_nil(&mut self) {
		self.kind = VAL_NIL;
		self.value.integer = 0;
	}

	/// @TODO:
	#[must_use]
	pub const fn is_nil(&self) -> bool {
		self.kind == VAL_NIL
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_bool(&self) -> Result<bool, Error> {
		if self.kind == VAL_BOOL {
			Ok(unsafe { self.value.boolean })
		} else {
			Err(Error::NoBoolean)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_bool(boolean: bool) -> Self {
		Self {
			kind: VAL_BOOL,
			value: InnerValue { boolean },
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn is_bool(&self) -> bool {
		self.kind == VAL_BOOL
	}

	/// @TODO:
	pub const fn to_bool(&mut self, boolean: bool) {
		self.kind = VAL_BOOL;
		self.value.boolean = boolean;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_double(&self) -> Result<f64, Error> {
		if self.kind == VAL_DOUBLE {
			Ok(unsafe { self.value.double })
		} else {
			Err(Error::NoDouble)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_double(double: f64) -> Self {
		Self {
			kind: VAL_DOUBLE,
			value: InnerValue { double },
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn is_double(&self) -> bool {
		self.kind == VAL_DOUBLE
	}

	/// @TODO:
	pub const fn to_double(&mut self, double: f64) {
		self.kind = VAL_DOUBLE;
		self.value.double = double;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_integer(&self) -> Result<i64, Error> {
		if self.kind == VAL_INT {
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
			kind: VAL_INT,
			value: InnerValue { integer },
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn is_integer(&self) -> bool {
		self.kind == VAL_INT
	}

	/// @TODO:
	pub const fn to_integer(&mut self, integer: i64) {
		self.kind = VAL_INT;
		self.value.integer = integer;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_string_pos(&self) -> Result<usize, Error> {
		if self.kind == VAL_STR {
			Ok(unsafe { self.value.string_pos })
		} else {
			Err(Error::NoString)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_string_pos(string_pos: usize) -> Self {
		Self {
			kind: VAL_STR,
			value: InnerValue { string_pos },
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn is_string_pos(&self) -> bool {
		self.kind == VAL_STR
	}

	/// @TODO:
	pub const fn to_string_pos(&mut self, string_pos: usize) {
		self.kind = VAL_STR;
		self.value.string_pos = string_pos;
	}
}
