// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! Activity interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use core::fmt::Debug;

use crate::Operational;
// endregion:	--- modules

// region:		--- Activity
/// Contract for an `Activity`
pub trait Activity: Debug + Operational + Send + Sync {
	/// Get [`Activity`]s id
	fn id(&self) -> String;

	/// Get [`Activity`]s id
	fn set_id(&mut self, id: String);
}
// endregion:	--- Activity

#[cfg(test)]
mod tests {
	use crate::{operational::Transitions, OperationState};
	use alloc::{boxed::Box, string::String};

	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Box<dyn Activity>>();
	}

	// #[dimas::activity(attr = "wrong attribute")]
	// #[derive(Debug)]
	// struct TestActivity {}

	// #[dimas::activity]
	// const fn test_fn() {}

	// #[dimas::activity]
	// struct Test();

	#[dimas_macros::activity]
	struct TestActivity {
		dummy: String,
		another: i32,
	}

	impl TestActivity {
		fn dummy(&self) -> &str {
			&self.dummy
		}

		const fn another(&self) -> i32 {
			self.another
		}
	}

	impl Transitions for TestActivity {}

	#[test]
	fn activity() {
		let mut activity = TestActivity::default();
		assert_eq!(activity.dummy(), "");
		assert_eq!(activity.id(), "");
		activity.set_id("new id".into());
		assert_eq!(activity.id(), "new id");
	}
}
