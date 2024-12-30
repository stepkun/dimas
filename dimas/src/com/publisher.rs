// Copyright © 2024 Stephan Kunz

//! Publisher

// region:      --- modules
use anyhow::Result;
use dimas_config::factory::BTFactory;
use dimas_core::behavior::{BehaviorCategory, BehaviorResult, BehaviorStatus};
use dimas_macros::{behavior, register_action};
// endregion:   --- modules

// region:      --- behavior
/// SyncAction "Publisher"
#[behavior(SyncAction)]
pub struct Publisher {}

#[allow(clippy::use_self)]
#[behavior(SyncAction)]
impl Publisher {
	/// @TODO:
	async fn tick(&self) -> BehaviorResult {
		println!("ticking Publisher");
		Ok(BehaviorStatus::Success)
	}

	/// Registration function
	pub fn register(factory: &mut BTFactory) {
		register_action!(factory, "Publisher", Publisher);
	}
}
// endregion:   --- behavior
