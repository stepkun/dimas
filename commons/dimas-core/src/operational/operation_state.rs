// Copyright © 2024 Stephan Kunz

//! Operational states of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, string::ToString, vec::Vec};
use bitcode::{Decode, Encode};
use core::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Error;
#[cfg(doc)]
use super::Operational;
// endregion:	--- modules

// region:		--- OperationState
/// The possible states an [`Operational`] entity can take
#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Debug, Default, Decode, Encode, Eq, PartialEq, Ord, PartialOrd)]
pub enum OperationState {
	/// Entity is in an erronous state
	Error = -2,
	/// Entity is not initialized
	#[default]
	Undefined = -1,
	/// Entity is in initial state
	Created = 0,
	/// Entity is setup properly
	Configured,
	/// Entity is listening and reacting only to important messages
	Inactive,
	/// Entity has full situational awareness (sensing) but does not act
	Standby,
	/// Entity is fully operational
	Active,
}

impl Add<i32> for OperationState {
	type Output = Self;

	/// add operation for [`OperationState`]
	/// # Panics
	/// if you try to add to much to the state
	fn add(self, rhs: i32) -> Self::Output {
		let value: i32 = self.into();
		Self::try_from(value + rhs).expect("addition to OperationState out of bounds")
	}
}

/// add and assign operation for [`OperationState`]
/// # Panics
/// if you try to add to much to the state
impl AddAssign<i32> for OperationState {
	fn add_assign(&mut self, rhs: i32) {
		*self = self.add(rhs);
	}
}

impl Display for OperationState {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Error => write!(f, "Error"),
			Self::Undefined => write!(f, "Undefined"),
			Self::Created => write!(f, "Created"),
			Self::Configured => write!(f, "Configured"),
			Self::Inactive => write!(f, "Inactive"),
			Self::Standby => write!(f, "Standby"),
			Self::Active => write!(f, "Active"),
		}
	}
}

impl Sub<Self> for OperationState {
	type Output = i32;

	/// sub operation for  2 [`OperationState`]s
	/// returns an [`i32`] difference value
	fn sub(self, rhs: Self) -> Self::Output {
		let left: i32 = self.into();
		let right: i32 = rhs.into();
		left - right
	}
}

impl Sub<i32> for OperationState {
	type Output = Self;

	/// sub operation for [`OperationState`]
	/// # Panics
	/// if you try to subtract to much from the state
	fn sub(self, rhs: i32) -> Self::Output {
		let value: i32 = self.into();
		Self::try_from(value - rhs).expect("subtraction from OperationState out of bounds")
	}
}

impl SubAssign<i32> for OperationState {
	fn sub_assign(&mut self, rhs: i32) {
		*self = self.sub(rhs);
	}
}

impl From<OperationState> for i32 {
	fn from(value: OperationState) -> Self {
		match value {
			OperationState::Error => -2,
			OperationState::Undefined => -1,
			OperationState::Created => 0,
			OperationState::Configured => 1,
			OperationState::Inactive => 2,
			OperationState::Standby => 3,
			OperationState::Active => 4,
		}
	}
}

impl TryFrom<i32> for OperationState {
	type Error = Box<dyn core::error::Error + Send + Sync + 'static>;

	fn try_from(value: i32) -> Result<Self, Box<dyn core::error::Error + Send + Sync + 'static>> {
		match value {
			-2 => Ok(Self::Error),
			-1 => Ok(Self::Undefined),
			0 => Ok(Self::Created),
			1 => Ok(Self::Configured),
			2 => Ok(Self::Inactive),
			3 => Ok(Self::Standby),
			4 => Ok(Self::Active),
			_ => Err(Box::new(Error::ParseInt { value })),
		}
	}
}

impl TryFrom<&str> for OperationState {
	type Error = Box<dyn core::error::Error + Send + Sync + 'static>;

	fn try_from(
		value: &str,
	) -> core::result::Result<Self, Box<dyn core::error::Error + Send + Sync + 'static>> {
		let v = value.to_lowercase();
		match v.as_str() {
			"created" => Ok(Self::Created),
			"configured" => Ok(Self::Configured),
			"inactive" => Ok(Self::Inactive),
			"standby" => Ok(Self::Standby),
			"active" => Ok(Self::Active),
			_ => Err(Error::UnknownOperationState {
				state: value.to_string(),
			}
			.into()),
		}
	}
}
// endregion:	--- OperationState

#[cfg(test)]
mod tests {
	#[doc(hidden)]
	extern crate std;

	use super::*;
	use std::panic::catch_unwind;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<OperationState>();
	}

	#[test]
	fn add() {
		assert_eq!(OperationState::Created + 1, OperationState::Configured);
		assert_eq!(OperationState::Configured + 1, OperationState::Inactive);
		assert_eq!(OperationState::Inactive + 1, OperationState::Standby);
		assert_eq!(OperationState::Standby + 1, OperationState::Active);
		assert_eq!(OperationState::Created + 4, OperationState::Active);
	}

	#[test]
	fn add_assign() {
		let mut state = OperationState::Created;
		state += 1;
		assert_eq!(&state, &OperationState::Configured);
		state += 1;
		assert_eq!(&state, &OperationState::Inactive);
		state += 1;
		assert_eq!(&state, &OperationState::Standby);
		state += 1;
		assert_eq!(&state, &OperationState::Active);

		state = OperationState::Created;
		state += 4;
		assert_eq!(&state, &OperationState::Active);
	}

	#[test]
	fn failing_add() {
		assert!(catch_unwind(|| OperationState::Active + 1).is_err());
		assert!(catch_unwind(|| OperationState::Created + 5).is_err());
	}

	#[test]
	fn sub() {
		assert_eq!(OperationState::Active - 1, OperationState::Standby);
		assert_eq!(OperationState::Standby - 1, OperationState::Inactive);
		assert_eq!(OperationState::Inactive - 1, OperationState::Configured);
		assert_eq!(OperationState::Configured - 1, OperationState::Created);
		assert_eq!(OperationState::Active - 4, OperationState::Created);
	}

	#[test]
	fn sub_assign() {
		let mut state = OperationState::Active;
		state -= 1;
		assert_eq!(&state, &OperationState::Standby);
		state -= 1;
		assert_eq!(&state, &OperationState::Inactive);
		state -= 1;
		assert_eq!(&state, &OperationState::Configured);
		state -= 1;
		assert_eq!(&state, &OperationState::Created);

		state = OperationState::Active;
		state -= 4;
		assert_eq!(&state, &OperationState::Created);
	}

	#[test]
	fn failing_sub() {
		assert!(catch_unwind(|| OperationState::Created - 3).is_err());
		assert!(catch_unwind(|| OperationState::Active - 7).is_err());
	}
}
