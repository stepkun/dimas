// Copyright © 2025 Stephan Kunz

//! Built in Always behavior of `DiMAS`
//! 

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
//endregion:    --- modules

/// The `Always` behavior returns Failure, Running or Success depending on the
/// stored [`BehaviorState`] value.
#[derive(Behavior, Debug, Default)]
pub struct Always {
	state: BehaviorState,
}

#[async_trait::async_trait]
impl BehaviorInstance for Always {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		Ok(self.state)
	}
}

impl BehaviorStatic for Always {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

impl Always {
	/// Constructor with arguments.
	#[must_use]
	pub const fn new(state: BehaviorState) -> Self {
		Self { state }
	}
	/// Initialization function.
	pub fn initialize(&mut self, state: BehaviorState) {
		self.state = state;
	}
}
