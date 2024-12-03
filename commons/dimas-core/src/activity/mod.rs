// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

//! core traits
//!

mod activity;
mod activity_type;
mod error;

// flatten
pub use activity::Activity;
pub use activity_type::ActivityType;
