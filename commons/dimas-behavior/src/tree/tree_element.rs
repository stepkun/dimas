// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTreeElement`]
//!

use alloc::string::ToString;
// region:      --- modules
use dimas_core::ConstString;
use dimas_scripting::{Error, SharedRuntime};

use crate::{
	behavior::{
		BehaviorData, BehaviorPtr, BehaviorResult, BehaviorState,
		error::BehaviorError,
		pre_post_conditions::{Conditions, PostConditions, PreConditions},
	},
	tree::tree_iter::TreeIter,
};

use super::BehaviorTreeElementList;
// endregion:   --- modules

// region:		--- TreeElementKind
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
/// @TODO:
pub enum TreeElementKind {
	/// A behavior tree leaf.
	Leaf,
	/// A behavior tree node.
	Node,
	/// A behavior subtree.
	SubTree,
}
//endregion:	--- TreeElementKind

// region:		--- BehaviorTreeElement
/// A tree elements.
pub struct BehaviorTreeElement {
	/// Kind of the element.
	kind: TreeElementKind,
	/// The behavior of that element.
	behavior: BehaviorPtr,
	/// Data of the Behavior.
	data: BehaviorData,
	/// Children of the element.
	children: BehaviorTreeElementList,
	/// Pre conditions, checked before a tick.
	pre_conditions: PreConditions,
	/// Post conditions, checked after a tick.
	post_conditions: PostConditions,
}

impl BehaviorTreeElement {
	/// Construct a [`BehaviorTreeElement`].
	/// Non public to enforce using the dedicated creation functions.
	#[allow(clippy::too_many_arguments)]
	#[inline]
	fn new(
		kind: TreeElementKind,
		behavior: BehaviorPtr,
		mut data: BehaviorData,
		children: BehaviorTreeElementList,
		conditions: Conditions,
	) -> Self {
		let groot2_path = match kind {
			TreeElementKind::Leaf | TreeElementKind::Node => data.description().path().clone(),
			TreeElementKind::SubTree => {
				if data.description().path().is_empty() {
					data.description().path().clone()
				} else {
					let uid = data.uid().to_string();
					(data.description().name().to_string() + "::" + &uid).into()
				}
			}
		};
		data.description_mut()
			.set_groot2_path(groot2_path);
		Self {
			kind,
			behavior,
			data,
			children,
			pre_conditions: conditions.pre,
			post_conditions: conditions.post,
		}
	}

	/// Create a tree leaf.
	#[must_use]
	pub(crate) fn create_leaf(data: BehaviorData, behavior: BehaviorPtr, conditions: Conditions) -> Self {
		Self::new(
			TreeElementKind::Leaf,
			behavior,
			data,
			BehaviorTreeElementList::default(),
			conditions,
		)
	}

	/// Create a tree node.
	#[must_use]
	pub(crate) fn create_node(
		data: BehaviorData,
		children: BehaviorTreeElementList,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self::new(TreeElementKind::Node, behavior, data, children, conditions)
	}

	/// Create a subtree.
	#[must_use]
	pub(crate) fn create_subtree(
		data: BehaviorData,
		children: BehaviorTreeElementList,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self::new(TreeElementKind::SubTree, behavior, data, children, conditions)
	}

	/// Get the uid.
	#[must_use]
	pub const fn uid(&self) -> u16 {
		self.data.uid()
	}

	/// Get a reference to the [`BehaviorData`].
	#[must_use]
	pub const fn data(&self) -> &BehaviorData {
		&self.data
	}

	/// Get a mutable reference to the [`BehaviorData`].
	#[must_use]
	pub const fn data_mut(&mut self) -> &mut BehaviorData {
		&mut self.data
	}

	/// Get a reference to the behavior.
	#[must_use]
	pub fn behavior(&self) -> &BehaviorPtr {
		&self.behavior
	}

	/// Get a mutable reference to the behavior.
	pub fn behavior_mut(&mut self) -> &mut BehaviorPtr {
		&mut self.behavior
	}

	/// Get the children.
	#[must_use]
	pub const fn children(&self) -> &BehaviorTreeElementList {
		&self.children
	}

	/// Get the children mutable.
	pub const fn children_mut(&mut self) -> &mut BehaviorTreeElementList {
		&mut self.children
	}

	/// Get the pre conditions.
	#[must_use]
	pub const fn pre_conditions(&self) -> &PreConditions {
		&self.pre_conditions
	}

	/// Get the post conditions.
	#[must_use]
	pub const fn post_conditions(&self) -> &PostConditions {
		&self.post_conditions
	}

	/// Halt the element and all its children.
	/// # Errors
	#[allow(clippy::unused_async)]
	pub async fn execute_halt(&mut self, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		self.halt(0, runtime)?;
		if let Some(script) = self.post_conditions.get("_onHalted") {
			let _ = runtime
				.lock()
				.run(script, self.data.blackboard_mut())?;
		}
		self.data.set_state(BehaviorState::Idle);
		Ok(())
	}

	/// Tick the element and its children.
	/// # Errors
	pub async fn execute_tick(&mut self, runtime: &SharedRuntime) -> BehaviorResult {
		// A pre-condition may return the next state which will override the current tick().
		let state = if let Some(result) = self.check_pre_conditions(runtime).await? {
			result
		} else if self.data.state() == BehaviorState::Idle {
			self.behavior
				.start(&mut self.data, &mut self.children, runtime)
				.await?
		} else {
			self.behavior
				.tick(&mut self.data, &mut self.children, runtime)
				.await?
		};

		self.check_post_conditions(state, runtime);

		// Preserve the last state if skipped, but communicate `Skipped` to parent
		if state != BehaviorState::Skipped {
			self.data.set_state(state);
		}

		Ok(state)
	}

	/// Halt child at `index`.
	/// # Errors
	/// - if index is out of childrens bounds.
	pub fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt_child(index)
	}

	/// Halt all children at and beyond `index`.
	/// # Errors
	/// - if index is out of childrens bounds.
	pub fn halt(&mut self, index: usize, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		self.children.halt(index, runtime)
	}

	/// Halt all children at and beyond `index`.
	/// # Errors
	/// - if index is out of childrens bounds.
	pub async fn reset(&mut self, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		self.children.reset(runtime).await
	}

	/// Add a pre state change callback with the given name.
	/// The name is not unique, which is important when removing callback.
	pub fn add_pre_state_change_callback<T>(&mut self, name: ConstString, callback: T)
	where
		T: Fn(&BehaviorData, &mut BehaviorState) + Send + Sync + 'static,
	{
		self.data
			.add_pre_state_change_callback(name, callback);
	}

	/// Remove any pre state change callback with the given name.
	pub fn remove_pre_state_change_callback(&mut self, name: &ConstString) {
		self.data.remove_pre_state_change_callback(name);
	}

	/// Return an iterator over the children.
	#[must_use]
	pub fn children_iter(&self) -> impl DoubleEndedIterator<Item = &Self> {
		self.children().iter()
	}

	/// Return a mutable iterator over the children.
	#[must_use]
	pub fn children_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
		self.children_mut().iter_mut()
	}

	async fn check_pre_conditions(&mut self, runtime: &SharedRuntime) -> Result<Option<BehaviorState>, Error> {
		if self.pre_conditions.is_some() {
			// Preconditions only applied when the node state is `Idle` or `Skipped`
			if self.data.state() == BehaviorState::Idle || self.data.state() == BehaviorState::Skipped {
				if let Some(script) = self.pre_conditions.get("_failureif") {
					let res = runtime
						.lock()
						.run(script, self.data.blackboard_mut())?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Failure));
					}
				}
				if let Some(script) = self.pre_conditions.get("_successif") {
					let res = runtime
						.lock()
						.run(script, self.data.blackboard_mut())?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Success));
					}
				}
				if let Some(script) = self.pre_conditions.get("_skipif") {
					let res = runtime
						.lock()
						.run(script, self.data.blackboard_mut())?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Skipped));
					}
				}
				if let Some(script) = self.pre_conditions.get("_while") {
					let res = runtime
						.lock()
						.run(script, self.data.blackboard_mut())?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Skipped));
					}
				}
			} else
			// Preconditions only applied when the node state is `Running`
			if self.data.state() == BehaviorState::Running {
				if let Some(script) = self.pre_conditions.get("_while") {
					let res = runtime
						.lock()
						.run(script, self.data.blackboard_mut())?;
					// if not true halt element and return `Skipped`
					if res.is_bool() && !res.as_bool()? {
						let _res = self.execute_halt(runtime).await;
						return Ok(Some(BehaviorState::Skipped));
					}
				}
			}
		}
		Ok(None)
	}

	fn check_post_conditions(&mut self, state: BehaviorState, runtime: &SharedRuntime) {
		if self.post_conditions.is_some() {
			match state {
				BehaviorState::Failure => {
					if let Some(script) = self.post_conditions.get("_onFailure") {
						let _: Result<dimas_scripting::execution::ScriptingValue, dimas_scripting::Error> = runtime
							.lock()
							.run(script, self.data.blackboard_mut());
					}
				}
				BehaviorState::Success => {
					if let Some(script) = self.post_conditions.get("_onSuccess") {
						let _ = runtime
							.lock()
							.run(script, self.data.blackboard_mut());
					}
				}
				// rest is ignored
				_ => {}
			}
			if let Some(script) = self.post_conditions.get("_post") {
				let _ = runtime
					.lock()
					.run(script, self.data.blackboard_mut());
			}
		}
	}

	pub(crate) const fn kind(&self) -> TreeElementKind {
		self.kind
	}

	/// Get an iterator over the tree element.
	pub fn iter(&self) -> impl Iterator<Item = &Self> {
		TreeIter::new(self)
	}
}
// endregion:	--- BehaviorTreeElement
