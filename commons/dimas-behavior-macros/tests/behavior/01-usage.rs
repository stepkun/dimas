// Copyright © 2025 Stephan Kunz

//! Test correct usage of behavior derive macro `Behavior` 

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::behavior::{BehaviorInstance, BehaviorStatic};

#[derive(dimas_behavior_macros::Behavior, Debug, Default)]
struct TestBehavior;

#[async_trait::async_trait]
impl BehaviorInstance for TestBehavior {
	async fn tick(
		&mut self,
		_behavior: &mut dimas_behavior::behavior::BehaviorData,
		_children: &mut dimas_behavior::tree::BehaviorTreeElementList,
		_runtime: &dimas_scripting::runtime::SharedRuntime,
	) -> dimas_behavior::behavior::BehaviorResult {
        Ok(dimas_behavior::behavior::BehaviorState::Success)
    }
}

impl BehaviorStatic for TestBehavior {
	fn kind() -> dimas_behavior::behavior::BehaviorKind {
		dimas_behavior::behavior::BehaviorKind::Action
	}
}

// dummy main
fn main(){}
