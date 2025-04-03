// Copyright © 2024 Stephan Kunz

//! Subscriber

// region:      --- modules
use dimas_config::factory::BTFactory;
use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
use dimas_macros::{behavior, register_control};
// endregion:   --- modules

// region:      --- behavior
/// A [`Subscriber`]
#[behavior(Control)]
pub struct Subscriber {}

#[allow(clippy::use_self)]
#[behavior(Control)]
impl Subscriber {
	async fn on_start(&self) -> BehaviorResult {
		println!("starting Subscriber");
		Ok(BehaviorStatus::Running)
	}

	async fn on_running(&self) -> BehaviorResult {
		println!("ticking Subscriber");
		Ok(BehaviorStatus::Running)
	}

	async fn halt(&self) {
		bhvr_.reset_children().await;
		// let handle = self.handle.take();
		// if let Some(handle) = handle {
		// 	handle.abort();
		// };
		// @TODO: clarify which status is best
		bhvr_.set_status(BehaviorStatus::Success);
	}

	/// Registration function
	pub fn register(factory: &mut BTFactory) {
		register_control!(factory, "Subscriber", Subscriber,);
	}
}
// endregion:   --- behavior
