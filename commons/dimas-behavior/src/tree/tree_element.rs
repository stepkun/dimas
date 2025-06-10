// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTreeElement`]
//!

// region:      --- modules
use alloc::{boxed::Box, vec::Vec};
use dimas_core::ConstString;
use dimas_scripting::{Error, SharedRuntime};

use crate::{
	behavior::{
		BehaviorPtr, BehaviorResult, BehaviorState,
		error::BehaviorError,
		pre_post_conditions::{Conditions, PostConditions, PreConditions},
	},
	blackboard::SharedBlackboard,
};

use super::{BehaviorTreeElementList, BehaviorTreeElementTickCallback};
// endregion:   --- modules

// region:		--- BehaviorTreeElement
/// A tree elements.
pub struct BehaviorTreeElement {
	/// UID of the element within the [`BehaviorTree`](crate::tree::BehaviorTree).
	/// 65536 [`BehaviorTreeElement`]s in a [`BehaviorTree`](crate::tree::BehaviorTree) should be sufficient.
	/// The ordering of the uid is following the creation order by the [`XmlParser`](crate::factory::xml_parser::XmlParser).
	/// This should end up in a depth first ordering.
	uid: u16,
	/// Name of the element.
	name: ConstString,
	/// Path to the element.
	/// In contrast to BehaviorTree.CPP this path is fully qualified,
	/// which means that every level is denoted explicitly, including the tree root.
	path: ConstString,
	/// Current [`BehaviorState`] of the element.
	state: BehaviorState,
	/// Reference to the [`Blackboard`] for the element.
	blackboard: SharedBlackboard,
	/// The behavior of that element.
	behavior: BehaviorPtr,
	/// Children of the element.
	children: BehaviorTreeElementList,
	/// Pre conditions, checked before a tick.
	pre_conditions: PreConditions,
	/// Post conditions, checked after a tick.
	post_conditions: PostConditions,
	/// List of pre state change callbacks with an identifier.
	/// These callbacks can be used for observation of the [`BehaviorTreeElement`] and
	/// for manipulation of the resulting [`BehaviorState`] of a tick.
	pre_state_change_hooks: Vec<(ConstString, Box<BehaviorTreeElementTickCallback>)>,
}

impl BehaviorTreeElement {
	/// Construct a [`BehaviorTreeElement`].
	/// Non public to enforce using the dedicated creation functions.
	#[allow(clippy::too_many_arguments)]
	#[inline]
	fn new(
		uid: u16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self {
			uid,
			name: name.into(),
			path: path.into(),
			state: BehaviorState::Idle,
			blackboard,
			behavior,
			children,
			pre_conditions: conditions.pre,
			post_conditions: conditions.post,
			pre_state_change_hooks: Vec::new(),
		}
	}

	/// Create a tree leaf.
	#[must_use]
	pub(crate) fn create_leaf(
		uid: u16,
		name: &str,
		path: &str,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self::new(
			uid,
			name,
			path,
			BehaviorTreeElementList::default(),
			blackboard,
			behavior,
			conditions,
		)
	}

	/// Create a tree node.
	#[must_use]
	pub(crate) fn create_node(
		uid: u16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self::new(uid, name, path, children, blackboard, behavior, conditions)
	}

	/// Create a subtree.
	#[must_use]
	pub(crate) fn create_subtree(
		uid: u16,
		name: &str,
		path: &str,
		children: BehaviorTreeElementList,
		blackboard: SharedBlackboard,
		behavior: BehaviorPtr,
		conditions: Conditions,
	) -> Self {
		Self::new(uid, name, path, children, blackboard, behavior, conditions)
	}

	/// Get the uid.
	#[must_use]
	pub const fn uid(&self) -> u16 {
		self.uid
	}

	/// Get the name.
	#[must_use]
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Get the path.
	#[must_use]
	pub fn path(&self) -> &str {
		&self.path
	}

	/// Get the state.
	#[must_use]
	pub const fn state(&self) -> BehaviorState {
		self.state
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

	/// Get the blackboard.
	#[must_use]
	pub fn blackboard(&self) -> SharedBlackboard {
		self.blackboard.clone()
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

	/// Halt the element and all its children.
	/// # Errors
	#[allow(clippy::unused_async)]
	pub async fn execute_halt(&mut self, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		let result = self.halt(0, runtime);
		if let Some(chunk) = self.post_conditions.get_chunk("_onHalted") {
			let _ = runtime
				.lock()
				.execute(chunk, &mut self.blackboard)?;
		}
		self.state = BehaviorState::Idle;
		result
	}

	/// Tick the element and its children.
	/// # Errors
	pub async fn execute_tick(&mut self, runtime: &SharedRuntime) -> BehaviorResult {
		// A pre-condition may return the next state which will override the current tick().
		let mut state = if let Some(result) = self.check_pre_conditions(runtime).await? {
			result
		} else if self.state == BehaviorState::Idle {
			self.behavior
				.start(self.state, &mut self.blackboard, &mut self.children, runtime)
				.await?
		} else {
			self.behavior
				.tick(self.state, &mut self.blackboard, &mut self.children, runtime)
				.await?
		};

		self.check_post_conditions(state, runtime);

		// Callback after finishing tick before remembering & propagating state
		for (_, callback) in &self.pre_state_change_hooks {
			callback(self, &mut state);
		}

		// Preserve the last (`Idle`) state if skipped, but communicate `Skipped` to parent
		if state != BehaviorState::Skipped {
			self.state = state;
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
	pub fn reset(&mut self, runtime: &SharedRuntime) -> Result<(), BehaviorError> {
		self.children.reset(runtime)
	}

	/// Add a pre state change callback with the given name.
	/// The name is not unique, which is important when removing callback.
	pub fn add_pre_state_change_callback<T>(&mut self, name: ConstString, callback: T)
	where
		T: Fn(&Self, &mut BehaviorState) + Send + Sync + 'static,
	{
		self.pre_state_change_hooks
			.push((name, Box::new(callback)));
	}

	/// Remove any pre state change callback with the given name.
	pub fn remove_pre_state_change_callback(&mut self, name: &ConstString) {
		// first collect all subscriber with that name ...
		let mut indices = Vec::new();
		for (index, (cb_name, _)) in self.pre_state_change_hooks.iter().enumerate() {
			if cb_name == name {
				indices.push(index);
			}
		}
		// ... then remove them from vec
		for index in indices {
			let _ = self.pre_state_change_hooks.remove(index);
		}
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
			if self.state == BehaviorState::Idle || self.state == BehaviorState::Skipped {
				if let Some(chunk) = self.pre_conditions.get_chunk("_failureif") {
					let res = runtime
						.lock()
						.execute(chunk, &mut self.blackboard)?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Failure));
					}
				}
				if let Some(chunk) = self.pre_conditions.get_chunk("_successif") {
					let res = runtime
						.lock()
						.execute(chunk, &mut self.blackboard)?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Success));
					}
				}
				if let Some(chunk) = self.pre_conditions.get_chunk("_skipif") {
					let res = runtime
						.lock()
						.execute(chunk, &mut self.blackboard)?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Skipped));
					}
				}
				if let Some(chunk) = self.pre_conditions.get_chunk("_while") {
					let res = runtime
						.lock()
						.execute(chunk, &mut self.blackboard)?;
					if res.is_bool() && res.as_bool()? {
						return Ok(Some(BehaviorState::Skipped));
					}
				}
			} else
			// Preconditions only applied when the node state is `Running`
			if self.state == BehaviorState::Running {
				if let Some(chunk) = self.pre_conditions.get_chunk("_while") {
					let res = runtime
						.lock()
						.execute(chunk, &mut self.blackboard)?;
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
					if let Some(chunk) = self.post_conditions.get_chunk("_onFailure") {
						let _: Result<dimas_scripting::execution::ScriptingValue, dimas_scripting::Error> = runtime
							.lock()
							.execute(chunk, &mut self.blackboard);
					}
				}
				BehaviorState::Success => {
					if let Some(chunk) = self.post_conditions.get_chunk("_onSuccess") {
						let _ = runtime
							.lock()
							.execute(chunk, &mut self.blackboard);
					}
				}
				// rest is ignored
				_ => {}
			}
			if let Some(chunk) = self.post_conditions.get_chunk("_post") {
				let _ = runtime
					.lock()
					.execute(chunk, &mut self.blackboard);
			}
		}
	}
}
// endregion:	--- BehaviorTreeElement
