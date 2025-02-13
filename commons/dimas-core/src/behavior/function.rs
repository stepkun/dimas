// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `DiMAS` implementation for registering functions as behavior

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
};
use core::any::Any;
use futures::future::BoxFuture;

use crate::port::PortList;

use super::{
	Behavior, BehaviorCategory, BehaviorConfig, BehaviorData, BehaviorResult, BehaviorStatus,
	BehaviorType,
};
// endregion:   --- modules

// region:      --- types
/// Signature of the registered behavior function called by `BehaviorFunction`'s tick
type BhvrFn = fn() -> BehaviorResult;
//type TickFn = for<'a> fn(&'a mut BehaviorData) -> BoxFuture<'a, BehaviorResult>;
// endregion:   --- types

// region:      --- BehaviorFunction
/// Implementation resembles the macro generated struct code
pub struct BehaviorFunction {
	/// the function to be called on tick
	internal_tick_fn: BhvrFn,
}

/// Implementation resembles the macro generated impl code
impl BehaviorFunction {
	/// generated behavior creation function
	pub fn create_behavior(
		name: impl AsRef<str>,
		config: BehaviorConfig,
		internal_tick_fn: BhvrFn,
	) -> Behavior {
		let ctx = Self { internal_tick_fn };
		let bhvr_data = BehaviorData {
			name: name.as_ref().to_string(),
			type_str: String::from("Function"),
			bhvr_type: BehaviorType::SyncAction,
			bhvr_category: BehaviorCategory::Action,
			config,
			status: BehaviorStatus::Idle,
			children: ::alloc::vec::Vec::new(),
			ports_fn: Self::_ports,
		};
		Behavior {
			data: bhvr_data,
			context: ::alloc::boxed::Box::new(ctx),
			running_fn: Self::_tick,
			start_fn: Self::_tick,
			halt_fn: Self::_halt,
		}
	}

	#[allow(clippy::unwrap_used)]
	fn _tick<'a>(
		_bhvr_: &'a mut BehaviorData,
		ctx: &'a mut Box<dyn Any + Send + Sync>,
	) -> BoxFuture<'a, BehaviorResult> {
		Box::pin(async move {
			//let mut self_ = ctx.downcast_mut::<Self>().unwrap();
			let self_ = ctx.downcast_ref::<Self>().unwrap();
			//(self_.internal_tick_fn)(bhvr_).await
			(self_.internal_tick_fn)()
		})
	}
	fn _halt<'a>(
		_bhvr_: &'a mut BehaviorData,
		_ctx: &'a mut Box<dyn Any + Send + Sync>,
	) -> BoxFuture<'a, ()> {
		Box::pin(async move {})
	}
	fn _ports() -> PortList {
		PortList::new()
	}
}
// endregion:   --- BehaviorFunction
