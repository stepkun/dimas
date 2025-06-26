// Copyright © 2025 Stephan Kunz

//! Built in `AlwaysXxx` behavior of `DiMAS`
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::{BehaviorData, BehaviorError};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	tree::BehaviorTreeElementList,
};
//endregion:    --- modules

// region:		--- StateAfter
/// The `StateAfter` behavior returns the stored [`BehaviorState`] after a certain amount of ticks,
/// depending on the stored state and count value.
/// Until then Running is returned.
#[derive(Behavior, Debug, Default)]
pub struct StateAfter {
	/// The [`BehaviorState`] to return finally.
	state: BehaviorState,
	/// The amount of ticks after whih the state will be returned.
	count: u8,
	remaining: u8,
}

#[async_trait::async_trait]
impl BehaviorInstance for StateAfter {
	async fn halt(
		&mut self,
		_behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		self.remaining = 0;
		Ok(())
	}

	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.remaining = self.count;
		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		if self.remaining == 0 {
			// self.remaining self.count;
			behavior.set_state(self.state);
		} else {
			self.remaining -= 1;
			behavior.set_state(BehaviorState::Running);
		}
		Ok(behavior.state())
	}
}

impl BehaviorStatic for StateAfter {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}
}

impl StateAfter {
	/// Constructor with arguments.
	#[must_use]
	pub const fn new(state: BehaviorState, count: u8) -> Self {
		Self {
			state,
			count,
			remaining: count,
		}
	}

	/// Initialization function.
	pub const fn initialize(&mut self, state: BehaviorState, count: u8) {
		self.state = state;
		self.count = count;
		self.remaining = count;
	}
}
// endregion:	--- StateAfter
