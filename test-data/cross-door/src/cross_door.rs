// Copyright © 2025 Stephan Kunz

//! Test behaviors
//!

use std::{sync::Arc, thread, time::Duration};

use dimas_behavior::{
	factory::{NewBehaviorTreeFactory, error::Error},
	new_behavior::{BehaviorResult, NewBehaviorStatus},
};
use parking_lot::Mutex;

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
// endregion:	--- modules

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
	pub fn is_door_closed(&mut self) -> BehaviorResult {
		sleep_ms(200);
		if self.door_open {
			Ok(NewBehaviorStatus::Failure)
		} else {
			Ok(NewBehaviorStatus::Success)
		}
	}

	/// FAILURE if `door_locked` == true
	/// # Errors
	/// never
	pub fn open_door(&mut self) -> BehaviorResult {
		sleep_ms(500);
		if self.door_locked {
			Ok(NewBehaviorStatus::Failure)
		} else {
			self.door_open = true;
			Ok(NewBehaviorStatus::Success)
		}
	}

	/// SUCCESS if `door_open` == true
	/// # Errors
	/// never
	pub fn pass_through_door(&mut self) -> BehaviorResult {
		sleep_ms(500);
		if self.door_open {
			Ok(NewBehaviorStatus::Success)
		} else {
			Ok(NewBehaviorStatus::Failure)
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
			Ok(NewBehaviorStatus::Success)
		} else {
			Ok(NewBehaviorStatus::Failure)
		}
	}

	/// WILL always open a door
	/// # Errors
	/// never
	pub fn smash_door(&mut self) -> BehaviorResult {
		self.door_locked = false;
		self.door_open = true;
		// smash always works
		Ok(NewBehaviorStatus::Success)
	}

	/// Registration function for the `CrossDoor` interface
	/// # Errors
	pub fn register_nodes(&self, factory: &mut NewBehaviorTreeFactory) -> Result<(), Error> {
		// @TODO: replace the workaround with a solution!
		let cross_door1 = Arc::new(Mutex::new(Self::default()));
		let cross_door2 = cross_door1.clone();
		let cross_door3 = cross_door1.clone();
		let cross_door4 = cross_door1.clone();
		let cross_door5 = cross_door1.clone();
		factory.register_simple_action(
			"IsDoorClosed",
			Arc::new(move || cross_door1.lock().is_door_closed()),
		)?;
		factory
			.register_simple_action("OpenDoor", Arc::new(move || cross_door2.lock().open_door()))?;
		factory.register_simple_action(
			"PassThroughDoor",
			Arc::new(move || cross_door3.lock().pass_through_door()),
		)?;
		factory
			.register_simple_action("PickLock", Arc::new(move || cross_door4.lock().pick_lock()))?;
		factory.register_simple_action(
			"SmashDoor",
			Arc::new(move || cross_door5.lock().smash_door()),
		)?;

		Ok(())
	}
}
