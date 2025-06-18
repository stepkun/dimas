// Copyright © 2025 Stephan Kunz
#![allow(clippy::unnecessary_wraps)]
#![allow(unused)]

//! Cross door behaviors
//!

extern crate alloc;

use std::{thread, time::Duration};

use dimas_behavior::{
	behavior::{BehaviorResult, BehaviorState, BehaviorType},
	factory::{BehaviorTreeFactory, error::Error},
	register_behavior,
};

fn sleep_ms(millisecs: u64) {
	thread::sleep(Duration::from_millis(millisecs));
}

/// `CrossDoor` behavior interface
pub struct CrossDoor {
	door_open: bool,
	door_locked: bool,
	pick_attempts: u8,
}

impl Default for CrossDoor {
	fn default() -> Self {
		Self {
			door_open: false,
			door_locked: true,
			pick_attempts: 0,
		}
	}
}

impl CrossDoor {
	/// SUCCESS if `door_open` == true
	/// # Errors
	/// never
	pub fn is_door_closed(&self) -> BehaviorResult {
		sleep_ms(200);
		if self.door_open {
			Ok(BehaviorState::Failure)
		} else {
			Ok(BehaviorState::Success)
		}
	}

	/// FAILURE if `door_locked` == true
	/// # Errors
	/// never
	pub fn open_door(&mut self) -> BehaviorResult {
		sleep_ms(500);
		if self.door_locked {
			Ok(BehaviorState::Failure)
		} else {
			self.door_open = true;
			Ok(BehaviorState::Success)
		}
	}

	/// SUCCESS if `door_open` == true
	/// # Errors
	/// never
	pub fn pass_through_door(&self) -> BehaviorResult {
		sleep_ms(500);
		if self.door_open {
			Ok(BehaviorState::Success)
		} else {
			Ok(BehaviorState::Failure)
		}
	}

	/// After 3 attempts, will open a locked door
	/// # Errors
	/// never
	pub fn pick_lock(&mut self) -> BehaviorResult {
		sleep_ms(500);
		self.pick_attempts += 1;
		// succeed at 3rd attempt
		if self.pick_attempts > 3 {
			self.door_locked = false;
			self.door_open = true;
			Ok(BehaviorState::Success)
		} else {
			Ok(BehaviorState::Failure)
		}
	}

	/// Reset cross door
	pub const fn reset(&mut self) {
		self.door_open = false;
		self.door_locked = true;
		self.pick_attempts = 0;
	}

	/// Will always open a door
	/// # Errors
	/// never
	pub const fn smash_door(&mut self) -> BehaviorResult {
		self.door_locked = false;
		self.door_open = true;
		// smash always works
		Ok(BehaviorState::Success)
	}

	/// Registration function for the `CrossDoor` interface
	/// # Errors
	pub fn register_behaviors(factory: &mut BehaviorTreeFactory) -> Result<(), Error> {
		register_behavior!(
			factory,
			Self::default(),
			is_door_closed,
			"IsDoorClosed",
			BehaviorType::Condition,
			open_door,
			"OpenDoor",
			BehaviorType::Action,
			pass_through_door,
			"PassThroughDoor",
			BehaviorType::Action,
			pick_lock,
			"PickLock",
			BehaviorType::Action,
			smash_door,
			"SmashDoor",
			BehaviorType::Action,
		)?;

		Ok(())
	}
}
