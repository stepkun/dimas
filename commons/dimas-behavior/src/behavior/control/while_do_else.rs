// Copyright © 2025 Stephan Kunz

//! `WhileDoElse` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use dimas_scripting::SharedRuntime;

use crate as dimas_behavior;
use crate::{
	Behavior,
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType,
		error::BehaviorError,
	},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Fallback
/// The `Fallback` behavior is used to try different strategies until one succeeds.
/// If any child returns RUNNING, previous children will NOT be ticked again.
/// - If all the children return FAILURE, this node returns FAILURE.
/// - If a child returns RUNNING, this node returns RUNNING.
/// - If a child returns SUCCESS, stop the loop and return SUCCESS.
#[derive(Behavior, Debug, Default)]
pub struct WhileDoElse;

#[async_trait::async_trait]
impl BehaviorInstance for WhileDoElse {
	async fn start(
		&mut self,
		state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// check composition only once at start
        if !(2..=3).contains(&children.len()) {
            return Err(BehaviorError::Composition(
                "WhileDoElse must have either 2 or 3 children.".into(),
            ));
        }

		self.tick(state, blackboard, children, runtime).await
	}

	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// extern crate std;
		// std::println!("ticking WhileDoElse");
      	let children_count = children.len();

        let condition_status = children[0].execute_tick(runtime).await?;

        if matches!(condition_status, BehaviorState::Running) {
            return Ok(BehaviorState::Running);
        }

        let mut status = BehaviorState::Idle;

        match condition_status {
            BehaviorState::Success => {
                if children_count == 3 {
                    children.halt_child(2)?;
                }

                status = children[1].execute_tick(runtime).await?;
            }
            BehaviorState::Failure => match children_count {
                3 => {
                    children.halt_child(1)?;
                    status = children[2].execute_tick(runtime).await?;
                }
                2 => {
                    status = BehaviorState::Failure;
                }
                _ => {}
            },
            _ => {}
        }

        match status {
            BehaviorState::Running => Ok(BehaviorState::Running),
            status => {
                children.reset(runtime)?;
                Ok(status)
            }
        }
    }
}

impl BehaviorStatic for WhileDoElse {
	fn kind() -> BehaviorType {
		BehaviorType::Control
	}
}
// endregion:   --- Fallback
