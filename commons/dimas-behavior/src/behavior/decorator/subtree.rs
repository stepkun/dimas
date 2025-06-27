// Copyright © 2025 Stephan Kunz

//! `SubTree` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::behavior::BehaviorData;
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Subtree
/// A `Subtree` is a `Decorator` but with its own [`BehaviorType`].
#[derive(Behavior, Debug, Default)]
pub struct Subtree {}

#[async_trait::async_trait]
impl BehaviorInstance for Subtree {
	async fn start(
		&mut self,
		_behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// check composition only on start
		if children.len() != 1 {
			return Err(BehaviorError::Composition("SubTree must have a single child!".into()));
		}
		children[0].execute_tick(runtime).await
	}

	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		children[0].execute_tick(runtime).await
	}
}

impl BehaviorStatic for Subtree {
	fn kind() -> BehaviorKind {
		BehaviorKind::SubTree
	}
}
// endregion:   --- Subtree
