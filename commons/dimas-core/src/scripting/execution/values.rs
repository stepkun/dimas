// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::redundant_closure_for_method_calls)]

//! Value implementations for `DiMAS` scripting
//! `Numbers` are always f64 and `HexNumbers` are always i32

use alloc::{borrow::ToOwned, vec::Vec};

/// Definition of the `Value` type
pub type Value = f64;
/// Definition of the `HexValue` type
pub type HexValue = i64;

/// @TODO
#[derive(Default)]
pub struct Numbers {
	values: Vec<Value>,
}

impl Numbers {
	/// Add a value to the array and return it position
	pub fn write(&mut self, value: Value) -> usize {
		self.values.push(value);
		self.values.len() - 1
	}

	/// read the value at a position
	#[must_use]
	pub fn read(&self, offset: usize) -> Value {
		self.values
			.get(offset)
			.map_or_else(|| todo!(), |value| value.to_owned())
	}
}

/// @TODO
#[derive(Default)]
pub struct HexNumbers {
	values: Vec<HexValue>,
}

impl HexNumbers {
	/// Add a value to the array and return it position
	pub fn write(&mut self, value: HexValue) -> usize {
		self.values.push(value);
		self.values.len() - 1
	}

	/// read the value at a position
	#[must_use]
	pub fn read(&self, offset: usize) -> HexValue {
		self.values
			.get(offset)
			.map_or_else(|| todo!(), |value| value.to_owned())
	}
}
