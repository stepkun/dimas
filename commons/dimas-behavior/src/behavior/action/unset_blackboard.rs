// Copyright © 2025 Stephan Kunz

//! `SetBlackboard` behavior implementation
//!

// region:      --- modules
use alloc::string::String;
use alloc::{boxed::Box, string::ToString};
use core::fmt::Debug;
use core::marker::PhantomData;
use core::str::FromStr;
use dimas_scripting::SharedRuntime;

use crate::behavior::BehaviorData;
use crate::port::{PortList, strip_bb_pointer};
use crate::{self as dimas_behavior, input_port, port_list};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:      --- UnsetBlackboard
/// The [`UnsetBlackboard`] behavior is used to delete a value of type T
/// from the Blackboard specified via port `key`.
/// Will return Success whether the entry exists or not.
#[derive(Behavior, Debug, Default)]
pub struct UnsetBlackboard<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	phantom: PhantomData<T>,
}

#[async_trait::async_trait]
impl<T> BehaviorInstance for UnsetBlackboard<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let key = behavior.get::<String>("key")?;
		match strip_bb_pointer(&key) {
			Some(stripped_key) => {
				let _ = behavior.delete::<String>(&stripped_key);
			}
			None => {
				let _ = behavior.delete::<String>(&key);
			}
		}

		Ok(BehaviorState::Success)
	}
}

impl<T> BehaviorStatic for UnsetBlackboard<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(
			String,
			"key",
			"",
			"Key of the entry to remove"
		),]
	}
}
// endregion:   --- UnsetBlackboard
