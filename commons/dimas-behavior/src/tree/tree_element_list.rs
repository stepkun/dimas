// Copyright © 2025 Stephan Kunz

//! [`BehaviorTreeComponentList`] implementation.
//!

// region:      --- modules
use alloc::{format, vec::Vec};
use core::ops::{Deref, DerefMut};
use dimas_scripting::SharedRuntime;

use crate::behavior::error::BehaviorError;

use super::BehaviorTreeElement;
// endregion:   --- modules

// region:		--- BehaviorTreeElementList
/// A List of tree components.
#[derive(Default)]
pub struct BehaviorTreeElementList(Vec<BehaviorTreeElement>);

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
	pub fn reset(&mut self, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		for child in &mut self.0 {
			child.halt(0, runtime)?;
		}
		Ok(())
	}

	/// Halt child at and beyond index.
	/// # Errors
	/// - if halt of a child fails
	pub fn halt(&mut self, index: usize, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		for i in index..self.0.len() {
			self.0[i].halt(0, runtime)?;
		}
		Ok(())
	}

	/// Halt child at index.
	/// # Errors
	/// - if index is out of bounds
	/// - if halt of the child fails
	pub fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		// An empty children list is ok for Action and Condition
		if self.0.is_empty() {
			return Ok(());
		} else if index >= self.0.len() {
			let txt = format!("behavior tries to halt a non-existent child at index [{index}]");
			return Err(BehaviorError::Composition(txt.into()));
		}
		self.0[index].halt_child(0)
	}
}
// endregion:	--- BehaviorTreeElementList
