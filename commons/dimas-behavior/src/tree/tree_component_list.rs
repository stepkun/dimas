// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]

//! [`BehaviorTreeComponentList`] implementation.
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use crate::behavior::error::BehaviorError;

use super::{BehaviorTreeComponent, TreeElement};
// endregion:   --- modules

// region:		--- BehaviorTreeComponentList
/// A List of tree components.
#[derive(Default)]
pub struct BehaviorTreeComponentList(pub(crate) Vec<TreeElement>);

impl Deref for BehaviorTreeComponentList {
	type Target = Vec<TreeElement>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BehaviorTreeComponentList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl BehaviorTreeComponentList {
	/// Reset all children
	/// # Errors
	/// - if a child errors on `halt()`
	pub fn reset(&mut self) -> Result<(), BehaviorError> {
		let x = &mut self.0;
		for child in x {
			child.halt(0)?;
		}
		Ok(())
	}

	pub(crate) fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.0[index].halt(0)
	}

	pub(crate) fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.0[index].halt_child(0)
	}
}
// endregion:	--- BehaviorTreeComponentList
