// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Strings in blackboard of `DiMAS`

// region:      --- modules
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::{any::Any, convert::Infallible, fmt::Debug};
use thiserror::Error;

use crate::behavior::BTToString;
// endregion:   --- modules

// region:      --- macros
/// Macro for simplifying implementation of `FromString` for any type that implements `FromStr`.
///
/// The macro-based implementation works for any type that implements `FromStr`;
/// it calls `parse()` under the hood.
#[doc(hidden)]
macro_rules! impl_from_string {
    ( $($t:ty),* ) => {
        $(
            impl $crate::blackboard::string::FromString for $t
            {
                type Err = <$t as ::alloc::str::FromStr>::Err;

                fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err> {
                    value.as_ref().parse()
                }
            }
        ) *
    };
}

/// Macro for simplifying implementation of `IntoString` for any type implementing `Display`.
///
/// Also implements the trait for `Vec<T>` for each type, creating a `;` delimited string,
/// calling `into_string()` on the item type.
///
/// Implementation works for any type that implements `Display`; it calls `to_string()`.
/// However, for custom implementations, don't include in this macro.
#[doc(hidden)]
macro_rules! impl_into_string {
    ( $($t:ty),* ) => {
        $(
            impl $crate::basic_types::BTToString for $t {
                fn bt_to_string(&self) -> String {
                    self.to_string()
                }
            }

            impl $crate::basic_types::BTToString for Vec<$t> {
                fn bt_to_string(&self) -> String {
                    self
                    .iter()
                    .map(|x| x.bt_to_string())
                    .collect::<Vec<String>>()
                    .join(";")
                }
            }
        ) *
    };
}
// endregion:   --- macros

// region:      --- FromString
/// @TODO:
pub trait FromString
where
	Self: Sized,
{
	/// @TODO:
	type Err;

	/// @TODO:
	/// # Errors
	fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err>;
}

impl<T> FromString for Vec<T>
where
	T: FromString,
{
	type Err = <T as FromString>::Err;

	fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err> {
		value
			.as_ref()
			.split(';')
			.map(|x| T::from_string(x))
			.collect()
	}
}

impl_from_string!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl FromString for String {
	type Err = Infallible;

	fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err> {
		Ok(value.as_ref().to_string())
	}
}

/// @TODO:
#[derive(Error, Debug)]
pub enum ParseBoolError {
	/// @TODO:
	#[error("string wasn't one of the expected: 1/0, true/false, TRUE/FALSE")]
	ParseError,
}

impl FromString for bool {
	type Err = ParseBoolError;

	fn from_string(value: impl AsRef<str>) -> Result<Self, ParseBoolError> {
		match value.as_ref() {
			"1" | "true" | "TRUE" => Ok(true),
			"0" | "false" | "FALSE" => Ok(false),
			_ => Err(ParseBoolError::ParseError),
		}
	}
}
// endregion:   --- FromString

// region:      --- ParseStr
/// @TODO:
pub trait ParseStr<T> {
	/// @TODO:
	type Err;

	/// @TODO:
	/// # Errors
	fn parse_str(&self) -> Result<T, Self::Err>;
}

// Implements ParseStr<T> for all T that implements FromString
impl<T, U> ParseStr<T> for U
where
	T: FromString,
	U: AsRef<str>,
{
	type Err = <T as FromString>::Err;

	fn parse_str(&self) -> Result<T, Self::Err> {
		<T as FromString>::from_string(self)
	}
}
// endregion:   --- ParseStr

// region:      --- BBString
/// Trait that provides `strip_bb_pointer()` for all `AsRef<str>`,
/// which includes `String` and `&str`.
pub trait BlackboardString {
	/// @TODO:
	fn strip_bb_pointer(&self) -> Option<String>;

	/// @TODO:
	fn is_bb_pointer(&self) -> bool;
}

impl<T> BlackboardString for T
where
	T: AsRef<str> + Clone,
{
	fn strip_bb_pointer(&self) -> Option<String> {
		let str_ref = self.as_ref();

		// Is bb pointer
		if str_ref.starts_with('{') && str_ref.ends_with('}') {
			Some(
				str_ref
					.strip_prefix('{')
					.unwrap_or_else(|| todo!())
					.strip_suffix('}')
					.unwrap_or_else(|| todo!())
					.to_string(),
			)
		} else {
			None
		}
	}

	fn is_bb_pointer(&self) -> bool {
		let str_ref = self.as_ref();
		str_ref.starts_with('{') && str_ref.ends_with('}')
	}
}
// endregion:   --- BBString

// region:      -- AnyStringy
/// Supertrait for `Any + BTToString`
pub trait AnyStringy: Any + BTToString + Send {}

impl<T> AnyStringy for T where T: Any + BTToString + Send {}

impl Debug for (dyn AnyStringy) {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "AnyStringy {{ .. }}")
	}
}
// endregion:   --- AnyStringy
