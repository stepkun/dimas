// Copyright © 2024 Stephan Kunz

//! Strings in blackboard of `DiMAS`

// region:      --- modules
use alloc::{
	str::FromStr,
	string::{String, ToString},
};
use core::{any::Any, fmt::Debug};
// endregion:   --- modules

// region:      --- ParseStr
/// @TODO:
pub trait ParseStr<T> {
	/// @TODO:
	type Err;

	/// @TODO:
	/// # Errors
	fn parse_str(&self) -> Result<T, Self::Err>;
}

// Implements ParseStr<T> for all T that implements FromStr
impl<T, U> ParseStr<T> for U
where
	T: FromStr,
	U: AsRef<str>,
{
	type Err = <T as FromStr>::Err;

	fn parse_str(&self) -> Result<T, Self::Err> {
		<T as FromStr>::from_str(self.as_ref())
	}
}
// endregion:   --- ParseStr

// region:      --- BlackboardString
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
// endregion:   --- BlackboardString

// region:      -- AnyStringy
/// Supertrait for `Any + ToString`
pub trait AnyStringy: Any + ToString + Send {}

impl<T> AnyStringy for T where T: Any + ToString + Send {}

impl Debug for (dyn AnyStringy) {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "AnyStringy {{ .. }}")
	}
}
// endregion:   --- AnyStringy
