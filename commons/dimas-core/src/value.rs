// Copyright © 2025 Stephan Kunz

//! A universal `Value` type for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{string::String, sync::Arc};
use core::{
	any::Any,
	fmt::{Debug, Display, Formatter},
};

use crate::error::Error;
// endregion:	--- modules

// region:		--- types
/// Supertrait to satisfy the compiler
pub trait DynamicValue: Any + Debug + Sync + Send {}
// endregion:	--- types

// region:		--- Value
/// Value type to allow storing different kinds of values
#[derive(Clone, Debug)]
pub enum Value {
	/// Nil signals the absence of a `Value`
	Nil(),
	/// Boolean type
	Boolean(bool),
	/// Float 64bit
	Float64(f64),
	/// Integer 64bit
	Int64(i64),
	/// String type
	String(String),
	/// Other custom types
	Dynamic(Arc<dyn DynamicValue>),
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Nil() => write!(f, "nil"),
			Self::Boolean(val) => write!(f, "{val}"),
			Self::Float64(val) => write!(f, "{val}"),
			Self::Int64(val) => write!(f, "{val}"),
			Self::String(val) => write!(f, "{val}"),
			Self::Dynamic(val) => write!(f, "{val:?}"),
		}
	}
}

impl Value {
	/// create a `Nil` value
	#[must_use]
	pub const fn nil() -> Self {
		Self::Nil()
	}

	/// Return the boolean value
	/// # Errors
	/// if it is not a boolean type
	pub const fn as_bool(&self) -> Result<bool, Error> {
		match self {
			Self::Boolean(b) => Ok(*b),
			_ => Err(Error::NoBoolean),
		}
	}

	/// Return the boolean value
    #[must_use]
    pub const fn is_bool(&self) -> bool {
		matches!(self, Self::Boolean(_))
	}
}
// endregion:	--- Value
