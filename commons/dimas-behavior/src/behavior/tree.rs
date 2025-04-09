// Copyright © 2024 Stephan Kunz

//! `dimas` behavior tree

#[doc(hidden)]
extern crate alloc;

// @TODO: try to remove std lib
#[doc(hidden)]
extern crate std;

use core::marker::PhantomData;

// region:      --- modules
use crate::{
	behavior::{Behavior, BehaviorResult, BehaviorStatus},
	blackboard::Blackboard,
};
use alloc::vec;
use alloc::vec::Vec;

use super::error::BehaviorError;
// endregion:   --- modules

// region: 		--- TreeIter
/// Iterator over the [`BehaviorTree`]
/// @TODO:
struct TreeIter<'a> {
	/// stack to do a depth first search
	stack: Vec<&'a Behavior>,
	/// Lifetime marker
	marker: PhantomData<&'a Behavior>,
}

impl<'a> TreeIter<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a Behavior) -> Self {
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

impl<'a> Iterator for TreeIter<'a> {
	type Item = &'a Behavior;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::cast_possible_wrap)]
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(bhvr) = self.stack.pop() {
			// Push children in revers order to maintain left-to-right order
			for child in bhvr.children_iter().rev() {
				self.stack.push(child);
			}
			return Some(bhvr);
		}
		None
	}
}
// endregion:	--- TreeIter

// region: 		--- TreeIterMut
/// Mutuable Iterator over the [`BehaviorTree`]
/// @TODO:
struct TeeIterMut<'a> {
	/// stack to do a depth first search
	stack: Vec<*mut Behavior>,
	/// Lifetime marker
	marker: PhantomData<&'a mut Behavior>,
}

impl<'a> TeeIterMut<'a> {
	/// @TODO:
	#[must_use]
	pub fn new(root: &'a mut Behavior) -> Self {
		Self {
			stack: vec![root],
			marker: PhantomData,
		}
	}
}

#[allow(unsafe_code)]
impl<'a> Iterator for TeeIterMut<'a> {
	type Item = &'a mut Behavior;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(bhvr_ptr) = self.stack.pop() {
			// we know this pointer is valid since the iterator owns the traversal
			let bhvr = unsafe { &mut *bhvr_ptr };
			// Push children in revers order to maintain left-to-right order
			for child in bhvr.children_iter_mut().rev() {
				self.stack.push(child);
			}
			return Some(&mut *bhvr);
		}
		None
	}
}
// endregion:	--- TreeIterMut

// region:      --- BehaviorTree
enum TickOption {
	WhileRunning,
	ExactlyOnce,
	OnceUnlessWokenUp,
}

/// The behavior node tree
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct BehaviorTree {
	root: Behavior,
}

impl BehaviorTree {
	/// @TODO:
	#[must_use]
	pub const fn new(root: Behavior) -> Self {
		Self { root }
	}

	async fn tick_root(&mut self, opt: TickOption) -> BehaviorResult {
		let mut status = BehaviorStatus::Idle;

		while status == BehaviorStatus::Idle
			|| (matches!(opt, TickOption::WhileRunning)
				&& matches!(status, BehaviorStatus::Running))
		{
			status = self.root.execute_tick().await?;

			// Not implemented: Check for wake-up conditions and tick again if so

			if status.is_completed() {
				self.root.reset_status();
			}
		}

		Ok(status)
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_exactly_once(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::ExactlyOnce).await
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::OnceUnlessWokenUp)
			.await
	}

	/// @TODO:
	/// # Errors
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		self.tick_root(TickOption::WhileRunning).await
	}

	/// @TODO:
	#[must_use]
	pub fn root_blackboard(&self) -> Blackboard {
		self.root.config().blackboard().clone()
	}

	/// @TODO:
	/// # Errors
	/// if index is out of bounds
	pub fn subtree(&self, _index: usize) -> Result<(), BehaviorError> {
		todo!()
	}

	/// @TODO:
	pub async fn halt_tree(&mut self) {
		self.root.halt().await;
	}

	/// @TODO:
	pub fn iter(&self) -> impl Iterator<Item = &Behavior> {
		TreeIter::new(&self.root)
	}

	/// @TODO:
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Behavior> {
		TeeIterMut::new(&mut self.root)
	}

	// /// @TODO:
	// #[allow(clippy::iter_not_returning_iterator)]
	// pub fn into_iter(self) -> impl Iterator<Item = Behavior> {
	// 	todo!()
	// }
}

/*
impl<'a> IntoIterator for BehaviorTree<'a> {
	type Item = &'a Behavior;

	type IntoIter = Self::Item;

	fn into_iter(self) -> Self::IntoIter {
		todo!()
	}
}
*/
// endregion:   --- BehaviorTree
