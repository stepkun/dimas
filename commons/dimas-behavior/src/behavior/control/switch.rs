// Copyright © 2025 Stephan Kunz

//! `Switch` behavior implementation
//!

// region:      --- modules
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use dimas_scripting::SharedRuntime;

use crate::behavior::BehaviorData;
use crate::port::PortList;
use crate::{self as dimas_behavior, input_port};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic, error::BehaviorError},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- Switch
/// The `Switch` behavior is .
#[derive(Behavior, Debug)]
pub struct Switch<const T: u8> {
	/// Defaults to T
	cases: u8,
	/// Defaults to '-1'
	running_child_index: i32,
}

impl<const T: u8> Default for Switch<T> {
	fn default() -> Self {
		Self {
			cases: T,
			running_child_index: -1,
		}
	}
}

#[async_trait::async_trait]
impl<const T: u8> BehaviorInstance for Switch<T> {
	async fn halt(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> Result<(), BehaviorError> {
		for child in &mut **children {
			child.execute_halt(runtime).await?;
		}
		self.cases = T;
		self.running_child_index = -1;

		behavior.set_state(BehaviorState::Idle);
		Ok(())
	}

	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.running_child_index = -1;

		// check composition
		if children.len() != 1 {
			return Err(BehaviorError::Composition(
				"Wrong number of children in Switch behavior: must be (num_cases + 1)!".into(),
			));
		}

		self.tick(behavior, children, runtime).await
	}

	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// default match index
		let default_index = i32::from(T);
		let mut match_index = i32::from(T);
		if let Ok(var) = behavior.get::<String>("variable") {
			for i in 0..T {
				let key = String::from("case_") + &i.to_string();
				let x = behavior.get::<String>(&key)?;
				if var == x {
					match_index = i32::from(i);
					break;
				}
			}
		}

		// stop child, if it is not the one that should run
		if self.running_child_index > 0 && match_index != self.running_child_index && match_index <= default_index {
			#[allow(clippy::cast_sign_loss)]
			children[self.running_child_index as usize]
				.execute_halt(runtime)
				.await?;
		}

		#[allow(clippy::cast_sign_loss)]
		let state = children[match_index as usize]
			.execute_tick(runtime)
			.await?;

		if state == BehaviorState::Skipped {
			// if the matching child is Skipped, should default be executed or
			// return just Skipped? Going with the latter for now.
			self.running_child_index = -1;
		} else if state == BehaviorState::Running {
			self.running_child_index = match_index;
		} else {
			children.reset(runtime).await?;
			self.running_child_index = -1;
		}
		Ok(state)
	}
}

impl<const T: u8> BehaviorStatic for Switch<T> {
	fn kind() -> BehaviorKind {
		BehaviorKind::Control
	}

	fn provided_ports() -> PortList {
		let mut ports = PortList::default();
		let port = input_port!(String, "variable");
		ports.add(port).expect("snh");

		for i in 0..T {
			let name = String::from("case_") + &i.to_string();
			let port = input_port!(String, name.as_str());
			ports.add(port).expect("snh");
		}
		ports
	}
}
// endregion:   --- Fallback
