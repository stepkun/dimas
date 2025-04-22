// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Action behavior library
//!

pub mod inverter;
pub mod retry_until_successful;

// region:      --- modules
use crate::behavior::BehaviorInstanceMethods;
// endregion:   --- modules

// region:      --- DecoratorBehavior
/// Common methods for control behaviors.
pub trait DecoratorBehavior: BehaviorInstanceMethods {}
// endregion:   --- DecoratorBehavior
