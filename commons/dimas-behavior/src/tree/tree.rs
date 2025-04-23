// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(unused)]

//! [`BehaviorTree`] implementation.
//!
//! Implemenation is a [`composite pattern`](https://en.wikipedia.org/wiki/Composite_pattern)
//! together with a [`proxy pattern`](https://en.wikipedia.org/wiki/Proxy_pattern)
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::{self, Vec},
};
use core::{
	any::{Any, TypeId},
	ops::{Deref, DerefMut},
};
use dimas_core::ConstString;
use dimas_scripting::{Parser, VM};
use hashbrown::HashMap;
use parking_lot::Mutex;
use rustc_hash::FxBuildHasher;

use crate::{
	behavior::{
		BehaviorConfigurationData, BehaviorInstanceMethods, BehaviorResult, BehaviorStatus,
		BehaviorTickData, BehaviorTreeMethods, error::BehaviorError,
	},
	blackboard::Blackboard,
};

use super::{BehaviorSubTree, BehaviorTreeComponent, error::Error};
// endregion:   --- modules

// region:		--- helper
/// Helper function to print a (sub)tree recursively
#[cfg(feature = "std")]
pub fn print_tree(root_node: &dyn BehaviorTreeComponent) {
	std::println!("{}", root_node.id());
	print_recursively(0, root_node);
}

/// Helper function to print a (sub)tree recursively
/// Recursion function to print a (sub)tree recursively
/// # Errors
/// - Limit is a tree-depth of 127
#[cfg(feature = "std")]
#[allow(clippy::needless_pass_by_value)]
fn print_recursively(level: i8, node: &dyn BehaviorTreeComponent) -> Result<(), Error> {
	if level == i8::MAX {
		return Err(Error::Unexpected(
			"recursion limit reached".into(),
			file!().into(),
			line!(),
		));
	}

	let next_level = level + 1;
	let mut indentation = String::new();
	for _ in 0..next_level {
		indentation.push_str("   |");
	}
	for child in &**node.children() {
		std::println!("{}- {}", indentation, child.id());
		print_recursively(next_level, child.as_ref());
	}
	Ok(())
}

// endregion:	--- helper

// region:		--- BehaviorTree
/// A Tree of [`BehaviorTreeComponent`]s
#[derive(Default)]
pub struct BehaviorTree {
	pub(crate) root: Option<BehaviorSubTree>,
	pub(crate) subtrees: Vec<BehaviorSubTree>,
}

impl BehaviorTree {
	pub(crate) fn add_root(&mut self, root: BehaviorTreeNode) {
		self.root = Some(Arc::new(Mutex::new(root)));
	}

	pub(crate) fn add_subtree(&mut self, subtree: BehaviorTreeNode) {
		self.subtrees.push(Arc::new(Mutex::new(subtree)));
	}

	/// Get a (sub)tree where index 0 is root tree
	/// # Panics
	/// - if no root tree is set
	#[must_use]
	pub fn subtree(&self, index: usize) -> BehaviorSubTree {
		if index == 0 {
			self.root.as_ref().expect("snh)").clone()
		} else {
			self.subtrees[index - 1].clone()
		}
	}

	/// Pretty print the tree
	/// # Errors
	/// - if root tree is not yet set
	#[allow(clippy::option_if_let_else)]
	pub fn print(&self) -> Result<(), Error> {
		if let Some(node) = &self.root {
			std::println!("{}", node.lock().id());
			print_recursively(0, node.lock().as_ref())
		} else {
			Err(Error::RootNotFound("TODO!".into()))
		}
	}

	/// Ticks the tree until it finishes either with [`BehaviorStatus::Success`] or [`BehaviorStatus::Failure`]
	/// # Errors
	/// - if no root exists
	pub async fn tick_while_running(&mut self) -> BehaviorResult {
		let mut status = BehaviorStatus::Idle;

		if let Some(root) = &mut self.root {
			while status == BehaviorStatus::Idle || matches!(status, BehaviorStatus::Running) {
				status = root.lock().execute_tick()?;

				// Not implemented: Check for wake-up conditions and tick again if so

				if status.is_completed() {
					//root.reset_status();
				}
			}
			Ok(status)
		} else {
			Err(BehaviorError::RootNotFound("@TODO: 1".into()))
		}
	}

	/// Ticks the tree exactly once
	/// # Errors
	/// - if no root exists
	pub async fn tick_once(&mut self) -> BehaviorResult {
		self.root.as_ref().map_or_else(
			|| Err(BehaviorError::RootNotFound("@TODO: 2".into())),
			|root| root.lock().execute_tick(),
		)
	}
}
// endregion:	--- BehaviorTree

// region:		--- BehaviorTreeComponentList
/// A List of tree components
#[derive(Default)]
pub struct BehaviorTreeComponentList(Vec<Box<dyn BehaviorTreeComponent>>);

impl Deref for BehaviorTreeComponentList {
	type Target = Vec<Box<dyn BehaviorTreeComponent>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BehaviorTreeComponentList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl BehaviorTreeComponent for BehaviorTreeComponentList {
	fn id(&self) -> &'static str {
		"BehaviorTreeComponentList"
	}

	fn blackboard(&self) -> Blackboard {
		Blackboard::default()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		self
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		for item in &mut self.0 {
			item.execute_tick()?;
		}
		Ok(BehaviorStatus::Success)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > self.0.len() {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.0[index].execute_halt()
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > self.0.len() {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		for child in &mut *self.0 {
			child.execute_halt()?;
		}
		Ok(())
	}
}

impl BehaviorTreeComponentList {
	/// Reset all children
	/// # Errors
	pub fn reset(&mut self) -> Result<(), BehaviorError> {
		self.halt(0)
	}
}
// endregion:	--- BehaviorTreeComponentList

// region:		--- BehaviorTreeLeaf
/// Implementation of a trees leaf
pub struct BehaviorTreeLeaf {
	/// ID of the node
	id: ConstString,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// The behavior of that leaf
	behavior: Box<dyn BehaviorTreeMethods>,
	/// dummy children list
	children: BehaviorTreeComponentList,
}

impl BehaviorTreeComponent for BehaviorTreeLeaf {
	fn id(&self) -> &str {
		&self.id
	}

	fn blackboard(&self) -> Blackboard {
		self.tick_data.blackboard.clone()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		&self.children
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status;
		if status == BehaviorStatus::Idle {
			status = self
				.behavior
				.start(&mut self.tick_data, &mut self.children)?;
		} else {
			status = self
				.behavior
				.tick(&mut self.tick_data, &mut self.children)?;
		}
		self.tick_data.status = status;
		Ok(status)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.behavior.halt(&mut self.children)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.behavior.halt(&mut self.children)
	}
}

impl BehaviorTreeLeaf {
	/// Construct a [`BehaviorTreeNode`]
	#[must_use]
	pub fn new(
		id: &str,
		tick_data: BehaviorTickData,
		behavior: Box<dyn BehaviorTreeMethods>,
	) -> Self {
		Self {
			id: id.into(),
			tick_data,
			behavior,
			children: BehaviorTreeComponentList::default(),
		}
	}

	/// Create a `Box<dyn BehaviorTreeComponent>`]
	#[must_use]
	pub fn create(
		id: &str,
		tick_data: BehaviorTickData,
		behavior: Box<dyn BehaviorTreeMethods>,
	) -> Box<dyn BehaviorTreeComponent> {
		Box::new(Self::new(id, tick_data, behavior))
	}
}
// endregion:	--- BehaviorTreeLeaf

// region:		--- BehaviorTreeNode
/// Implementation of a trees node
pub struct BehaviorTreeNode {
	/// ID of the node
	id: ConstString,
	/// Children
	children: BehaviorTreeComponentList,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// The behavior of that leaf
	behavior: Box<dyn BehaviorTreeMethods>,
}

impl AsRef<dyn BehaviorTreeComponent + 'static> for BehaviorTreeNode {
	fn as_ref(&self) -> &(dyn BehaviorTreeComponent + 'static) {
		self
	}
}

impl BehaviorTreeComponent for BehaviorTreeNode {
	fn id(&self) -> &str {
		&self.id
	}

	fn blackboard(&self) -> Blackboard {
		self.tick_data.blackboard.clone()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		&self.children
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		let mut status = self.tick_data.status;
		if status == BehaviorStatus::Idle {
			status = self
				.behavior
				.start(&mut self.tick_data, &mut self.children)?;
		} else {
			status = self
				.behavior
				.tick(&mut self.tick_data, &mut self.children)?;
		}
		self.tick_data.status = status;
		Ok(status)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt_child(index)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		self.children.halt(index)
	}
}

impl BehaviorTreeNode {
	/// Construct a [`BehaviorTreeNode`]
	#[must_use]
	pub fn new(
		id: &str,
		children: BehaviorTreeComponentList,
		tick_data: BehaviorTickData,
		behavior: Box<dyn BehaviorTreeMethods>,
	) -> Self {
		Self {
			id: id.into(),
			children,
			tick_data,
			behavior,
		}
	}

	/// Create a `Box<dyn BehaviorTreeComponent>`]
	#[must_use]
	pub fn create(
		id: &str,
		children: BehaviorTreeComponentList,
		tick_data: BehaviorTickData,
		behavior: Box<dyn BehaviorTreeMethods>,
	) -> Box<dyn BehaviorTreeComponent> {
		Box::new(Self::new(id, children, tick_data, behavior))
	}

	/// Get the id
	#[must_use]
	pub const fn id(&self) -> &str {
		&self.id
	}
}
// endregion:	--- BehaviorTreeNode

// region:		--- BehaviorTreeProxy
/// Implementation of a trees proxy node
pub struct BehaviorTreeProxy {
	/// ID of the node
	id: ConstString,
	/// The Subtree to call
	subtree: Option<BehaviorSubTree>,
	/// Data needed in every tick
	tick_data: BehaviorTickData,
	/// dummy list
	children: BehaviorTreeComponentList,
}

impl BehaviorTreeComponent for BehaviorTreeProxy {
	fn id(&self) -> &str {
		&self.id
	}

	fn blackboard(&self) -> Blackboard {
		self.tick_data.blackboard.clone()
	}

	fn children(&self) -> &BehaviorTreeComponentList {
		&self.children
	}

	fn execute_tick(&mut self) -> BehaviorResult {
		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.lock().execute_tick(),
		)
	}

	fn halt_child(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > 0 {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.lock().execute_halt(),
		)
	}

	fn halt(&mut self, index: usize) -> Result<(), BehaviorError> {
		if index > 0 {
			return Err(BehaviorError::IndexOutOfBounds(index));
		}

		self.subtree.as_ref().map_or_else(
			|| {
				let msg = format!("Proxy [{}] w/o linked Subtree", &self.id).into();
				Err(BehaviorError::Composition(msg))
			},
			|subtree| subtree.lock().execute_halt(),
		)
	}
}

impl BehaviorTreeProxy {
	/// Construct a [`BehaviorTreeProxy`]
	#[must_use]
	pub fn new(id: &str, tick_data: BehaviorTickData) -> Self {
		Self {
			id: id.into(),
			subtree: None,
			tick_data,
			children: BehaviorTreeComponentList::default(),
		}
	}

	/// Create a `Box<dyn BehaviorTreeComponent>`]
	#[must_use]
	pub fn create(id: &str, tick_data: BehaviorTickData) -> Box<dyn BehaviorTreeComponent> {
		Box::new(Self::new(id, tick_data))
	}
}
// endregion:	--- BehaviorTreeProxy
