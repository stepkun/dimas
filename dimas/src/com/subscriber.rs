// Copyright © 2024 Stephan Kunz

//! Subscriber

// region:      --- modules
use anyhow::Result;
use dimas_config::factory::BTFactory;
use dimas_core::behavior::{BehaviorCategory, BehaviorResult, BehaviorStatus};
use dimas_macros::{behavior, register_action};
// endregion:   --- modules

// region:      --- behavior
/// Action "Subscriber"
#[behavior(Action)]
pub struct Subscriber {}

#[allow(clippy::use_self)]
#[behavior(Action)]
impl Subscriber {
	async fn on_start(&self) -> BehaviorResult {
		println!("starting Subscriber");
		Ok(BehaviorStatus::Running)
	}

	async fn on_running(&self) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorStatus::Running)
	}

	/// Registration function
	pub fn register(factory: &mut BTFactory) {
		register_action!(factory, "Subscriber", Subscriber,);
	}
}
// endregion:   --- behavior
