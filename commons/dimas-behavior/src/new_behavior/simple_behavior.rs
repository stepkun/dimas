// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]
#![allow(unused)]

//! `DiMAS` implementation for registering functions as behavior

// region:      --- modules
use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::{new_port::NewPortList, tree::BehaviorTreeComponent};

use super::{
	BehaviorAllMethods, BehaviorCreationFn, BehaviorInstanceMethods, BehaviorRedirectionMethods,
	BehaviorResult, BehaviorStaticMethods, BehaviorTickData, BehaviorTreeMethods,
	NewBehaviorStatus,
};
// endregion:   --- modules

// region:      --- types
/// Signature of a simple registered behavior function called by `SimpleBehavior`'s tick
pub type SimpleBhvrTickFn = Arc<dyn Fn() -> BehaviorResult + Send + Sync + 'static>;

#[allow(clippy::unnecessary_wraps)]
const fn simple_tick_fn() -> BehaviorResult {
	Ok(NewBehaviorStatus::Failure)
}

/// Signature of a registered behavior function called by `SimpleBehavior`'s tick
pub type ComplexBhvrTickFn =
	Arc<dyn Fn(&BehaviorTreeComponent) -> BehaviorResult + Send + Sync + 'static>;

#[allow(clippy::unnecessary_wraps)]
const fn full_tick_fn() -> BehaviorResult {
	Ok(NewBehaviorStatus::Failure)
}
// endregion:   --- types

// region:      --- BehaviorFunction
/// A simple behavior
pub struct SimpleBehavior {
	/// the function to be called on tick
	pub(crate) simple_tick_fn: Option<SimpleBhvrTickFn>,
	/// the function to be called on tick
	pub(crate) complex_tick_fn: Option<ComplexBhvrTickFn>,
	/// list of provided ports
	pub(crate) provided_ports: Option<NewPortList>,
}

impl core::fmt::Debug for SimpleBehavior {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SimpleBehavior")
			//.field("tick_fn", &self.tick_fn)
			.finish()
	}
}

impl BehaviorTreeMethods for SimpleBehavior {}

impl BehaviorInstanceMethods for SimpleBehavior {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		if self.provided_ports.is_some() {
			self
				.complex_tick_fn
				.as_ref()
				.expect("snh")(tree_node)
		} else {
			(self.simple_tick_fn.as_ref().expect("snh"))()
		}
	}
}

impl BehaviorRedirectionMethods for SimpleBehavior {
	#[allow(clippy::option_if_let_else)]
	fn static_provided_ports(&self) -> NewPortList {
		if let Some(port_list) = &self.provided_ports {
			port_list.clone()
		} else {
			NewPortList::default()
		}
	}
}

//impl BehaviorStaticMethods for SimpleBehavior {}

/// Implementation resembles the macro generated impl code
impl SimpleBehavior {
	/// Create a `SimpleBehavior` with the given function
	pub fn create(tick_fn: SimpleBhvrTickFn) -> Box<BehaviorCreationFn> {
		Box::new(move || {
			Box::new(Self {
				simple_tick_fn: Some(tick_fn.clone()),
				complex_tick_fn: None,
				provided_ports: None,
			})
		})
	}

	/// Create a `SimpleBehavior` with the given function and list of ports
	pub fn create_with_ports(
		tick_fn: ComplexBhvrTickFn,
		port_list: NewPortList,
	) -> Box<BehaviorCreationFn> {
		Box::new(move || {
			Box::new(Self {
				simple_tick_fn: None,
				complex_tick_fn: Some(tick_fn.clone()),
				provided_ports: Some(port_list.clone()),
			})
		})
	}
}
// endregion:   --- BehaviorFunction
