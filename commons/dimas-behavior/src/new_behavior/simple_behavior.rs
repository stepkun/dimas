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
};
// endregion:   --- modules

// region:      --- types
/// Signature of the registered behavior function called by `BehaviorFunction`'s tick
pub type BhvrTickFn = Arc<dyn Fn() -> BehaviorResult + Send + Sync + 'static>;
// endregion:   --- types

// region:      --- BehaviorFunction
/// A simple behavior
pub struct SimpleBehavior {
	/// the function to be called on tick
	tick_fn: BhvrTickFn,
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
		(self.tick_fn)()
	}
}

impl BehaviorRedirectionMethods for SimpleBehavior {
	fn static_provided_ports(&self) -> NewPortList {
		Self::provided_ports()
	}
}

impl BehaviorStaticMethods for SimpleBehavior {}

/// Implementation resembles the macro generated impl code
impl SimpleBehavior {
	/// Create a `SimpleBehavior` with the given function
	pub fn create(tick_fn: BhvrTickFn) -> Box<BehaviorCreationFn> {
		Box::new(move || {
			Box::new(Self {
				tick_fn: tick_fn.clone(),
			})
		})
	}
}
// endregion:   --- BehaviorFunction
