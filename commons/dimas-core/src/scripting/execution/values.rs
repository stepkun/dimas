// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::redundant_closure_for_method_calls)]

//! Value implementations for `DiMAS` scripting
//! `Numbers` are always f64 and `HexNumbers` are always i64

use core::fmt::Display;

use alloc::{borrow::ToOwned, vec::Vec};

use super::error::Error;

/// Constants for working with different types of [`Value`]s
pub(crate) const NIL: i8 = 0;
pub(crate) const BOOLEAN: i8 = NIL + 1;
pub(crate) const INTEGER: i8 = BOOLEAN + 1;
pub(crate) const HEX: i8 = BOOLEAN + 1;
pub(crate) const DOUBLE: i8 = INTEGER + 1;

/// The inner part of the `Value` type is a union of available types
#[derive(Clone, Copy)]
union InnerValue {
	boolean: bool,
	integer: i64,
	double: f64,
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
			kind: NIL,
			value: InnerValue { integer: 0 },
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.kind {
			BOOLEAN => write!(f, "{}", self.as_bool().expect("snh")),
			INTEGER => write!(f, "{}", self.as_integer().expect("snh")),
			DOUBLE => write!(f, "{}", self.as_double().expect("snh")),
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
			kind: NIL,
			value: InnerValue { integer: 0 },
		}
	}

	/// @TODO:
	pub const fn to_nil(&mut self) {
		self.kind = NIL;
		self.value.integer = 0;
	}

	/// @TODO:
	#[must_use]
	pub const fn is_nil(&self) -> bool {
		self.kind == NIL
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_bool(&self) -> Result<bool, Error> {
		if self.kind == BOOLEAN {
			Ok(unsafe { self.value.boolean })
		} else {
			Err(Error::NoBoolean)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_bool(boolean: bool) -> Self {
		Self {
			kind: BOOLEAN,
			value: InnerValue { boolean },
		}
	}

	/// @TODO:
	pub const fn to_bool(&mut self, boolean: bool) {
		self.kind = BOOLEAN;
		self.value.boolean = boolean;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_integer(&self) -> Result<i64, Error> {
		if self.kind == INTEGER {
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
			kind: INTEGER,
			value: InnerValue { integer },
		}
	}

	/// @TODO:
	pub const fn to_integer(&mut self, integer: i64) {
		self.kind = INTEGER;
		self.value.integer = integer;
	}

	/// @TODO:
	/// # Errors
	#[allow(unsafe_code)]
	pub const fn as_double(&self) -> Result<f64, Error> {
		if self.kind == DOUBLE {
			Ok(unsafe { self.value.double })
		} else {
			Err(Error::NoDouble)
		}
	}

	/// @TODO:
	#[must_use]
	pub const fn from_double(double: f64) -> Self {
		Self {
			kind: DOUBLE,
			value: InnerValue { double },
		}
	}

	/// @TODO:
	pub const fn to_double(&mut self, double: f64) {
		self.kind = DOUBLE;
		self.value.double = double;
	}
}
