// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]

//! Built in retry node of `DiMAS`

// region:      --- modules
use alloc::string::ToString;
use dimas_core::behavior::error::BehaviorError;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_core::port::PortList;
use dimas_core::{define_ports, input_port};
use dimas_macros::behavior;
//endregion:    --- modules

// The RetryNode is used to execute a child several times if it fails.
///
/// If the child returns SUCCESS, the loop is stopped and this node
/// returns SUCCESS.
///
/// If the child returns FAILURE, this node will try again up to N times
/// (N is read from port "num_attempts").
///
/// Example:
///
/// ```xml
/// <RetryUntilSuccessful num_attempts="3">
///     <OpenDoor/>
/// </RetryUntilSuccessful>
/// ```
#[behavior(SyncDecorator)]
pub struct Retry {
	#[bhvr(default = "-1")]
	max_attempts: i32,
	#[bhvr(default = "0")]
	try_count: usize,
	#[bhvr(default = "true")]
	all_skipped: bool,
}

#[behavior(SyncDecorator)]
impl Retry {
	async fn tick(&mut self) -> BehaviorResult {
		// Load num_cycles from the port value
		self.max_attempts = bhvr_.config.get_input("num_attempts")?;

		let mut do_loop = (self.try_count as i32) < self.max_attempts || self.max_attempts == -1;

		if matches!(bhvr_.status, BehaviorStatus::Idle) {
			self.all_skipped = true;
		}

		bhvr_.status = BehaviorStatus::Running;

		while do_loop {
			let child_status = bhvr_
				.child()
				.unwrap_or_else(|| todo!())
				.execute_tick()
				.await?;

			self.all_skipped &= matches!(child_status, BehaviorStatus::Skipped);

			match child_status {
				BehaviorStatus::Success => {
					self.try_count = 0;
					bhvr_.reset_child().await;

					return Ok(BehaviorStatus::Success);
				}
				BehaviorStatus::Failure => {
					self.try_count += 1;
					do_loop =
						(self.try_count as i32) < self.max_attempts || self.max_attempts == -1;

					bhvr_.reset_child().await;
				}
				BehaviorStatus::Running => return Ok(BehaviorStatus::Running),
				BehaviorStatus::Skipped => {
					bhvr_.reset_child().await;

					return Ok(BehaviorStatus::Skipped);
				}
				BehaviorStatus::Idle => {
					return Err(BehaviorError::Status(
						"InverterNode".to_string(),
						"Idle".to_string(),
					))
				}
			}
		}

		self.try_count = 0;

		if self.all_skipped {
			Ok(BehaviorStatus::Skipped)
		} else {
			Ok(BehaviorStatus::Failure)
		}
	}

	fn ports() -> PortList {
		define_ports!(input_port!("num_attempts"))
	}

	async fn halt(&mut self) {
		self.try_count = 0;
		bhvr_.reset_child().await;
	}
}
