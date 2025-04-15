// Copyright © 2025 Stephan Kunz
#![allow(dead_code)]
#![allow(unused)]

//! `DiMAS` implementation for registering functions as behavior

// region:      --- modules
use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::tree::BehaviorTreeComponent;

use super::{BehaviorCreationFn, BehaviorMethods, BehaviorResult, BehaviorTickData};
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

impl BehaviorMethods for SimpleBehavior {
	fn tick(
		&mut self,
		tick_data: &mut BehaviorTickData,
		children: &mut Vec<BehaviorTreeComponent>,
	) -> BehaviorResult {
		(self.tick_fn)()
	}
}

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
