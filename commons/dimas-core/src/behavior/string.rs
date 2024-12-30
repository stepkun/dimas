// Copyright © 2024 Stephan Kunz

//! `dimas-behaviortree` string helper

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{
	string::{String, ToString},
	vec::Vec,
};

use crate::{
	behavior::{BehaviorCategory, BehaviorStatus},
	port::PortDirection,
};
// endregion:   --- modules

// region:		--- macros
/// Macro for simplifying implementation of `IntoString` for any type implementing `Display`.
///
/// Also implements the trait for `Vec<T>` for each type, creating a `;` delimited string,
/// calling `into_string()` on the item type.
///
/// Implementation works for any type that implements `Display`; it calls `to_string()`.
/// However, for custom implementations, don't include in this macro.
macro_rules! impl_into_string {
    ( $($t:ty),* ) => {
        $(
            impl $crate::behavior::BTToString for $t {
                fn bt_to_string(&self) -> String {
                    self.to_string()
                }
            }

            impl $crate::behavior::BTToString for Vec<$t> {
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
// endregion:	--- macros

/// @TODO:
#[allow(clippy::module_name_repetitions)]
pub trait BTToString {
	/// @TODO:
	fn bt_to_string(&self) -> String;
}

impl BTToString for String {
	fn bt_to_string(&self) -> String {
		self.clone()
	}
}

impl_into_string!(
	u8,
	u16,
	u32,
	u64,
	u128,
	usize,
	i8,
	i16,
	i32,
	i64,
	i128,
	isize,
	f32,
	f64,
	bool,
	BehaviorStatus,
	BehaviorCategory,
	PortDirection,
	&str
);
