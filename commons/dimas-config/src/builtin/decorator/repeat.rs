// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in repeat node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_core::port::PortList;
use dimas_core::{define_ports, input_port};
use dimas_macros::behavior;
//endregion:    --- modules

/// The Repeat decorator is used to execute a child several times, as long
/// as it succeeds.
///
/// To succeed, the child must return SUCCESS N times (port "num_cycles").
///
/// If the child returns FAILURE, the loop is stopped and this node
/// returns FAILURE.
///
/// Example:
///
/// ```xml
/// <Repeat num_cycles="3">
///   <ClapYourHandsOnce/>
/// </Repeat>
/// ```
#[behavior(SyncDecorator)]
pub struct Repeat {
	#[bhvr(default = "-1")]
	num_cycles: i32,
	#[bhvr(default = "0")]
	repeat_count: usize,
	#[bhvr(default = "true")]
	all_skipped: bool,
}

#[behavior(SyncDecorator)]
impl Repeat {
	async fn tick(&mut self) -> BehaviorResult {
		// Load num_cycles from the port value
		self.num_cycles = bhvr_.config.get_input("num_cycles")?;

		let mut do_loop = (self.repeat_count as i32) < self.num_cycles || self.num_cycles == -1;

		if matches!(bhvr_.status, BehaviorStatus::Idle) {
			self.all_skipped = true;
		}

		bhvr_.status = BehaviorStatus::Running;

		if do_loop {
			let child_status = bhvr_
				.child()
				.unwrap_or_else(|| todo!())
				.execute_tick()
				.await?;

			self.all_skipped &= matches!(child_status, BehaviorStatus::Skipped);

			match child_status {
				BehaviorStatus::Success => {
					self.repeat_count += 1;
					bhvr_.reset_child().await;

					return Ok(BehaviorStatus::Running);
				}
				BehaviorStatus::Failure => {
					self.repeat_count = 0;
					bhvr_.reset_child().await;

					return Ok(BehaviorStatus::Failure);
				}
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Skipped => {
					bhvr_.reset_child().await;

					return Ok(BehaviorStatus::Skipped);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"Repeat Decorator".to_string(),
						"Idle".to_string(),
					));
				}
			}
		}

		// reset try counter
		self.repeat_count = 0;

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Success)
		}
	}

	fn ports() -> PortList {
		define_ports!(input_port!("num_cycles"))
	}

	async fn halt(&mut self) {
		self.repeat_count = 0;
		bhvr_.reset_child().await;
	}
}
