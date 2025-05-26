// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]

//! [`BehaviorTreeComponentList`] implementation.
//!

// region:      --- modules
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use crate::behavior::error::BehaviorError;

use super::{BehaviorTreeComponent, BehaviorTreeElement};
// endregion:   --- modules

// region:		--- BehaviorTreeElementList
/// A List of tree components.
#[derive(Default)]
pub struct BehaviorTreeElementList(pub(super) Vec<BehaviorTreeElement>);

impl Deref for BehaviorTreeElementList {
	type Target = Vec<BehaviorTreeElement>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BehaviorTreeElementList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl BehaviorTreeElementList {
	/// Reset all children
	/// # Errors
	/// - if a child errors on `halt()`
	pub fn reset(&mut self) -> Result<(), BehaviorError> {
		for child in &mut self.0 {
			child.halt(0)?;
		}
		Ok(())
	}

	/// Halt child at and beyond index.
	pub fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		for i in index..self.0.len() {
			self.0[i].halt(0)?;
		}
		Ok(())
	}

	/// Halt child at index.
	pub fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.0[index].halt_child(0)
	}
}
// endregion:	--- BehaviorTreeElementList
