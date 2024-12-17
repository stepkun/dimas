// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

//! Activity traits & structs
//!

mod activity;
mod activity_data;
mod activity_type;
mod error;

// flatten
pub use activity::{Activity, ActivityId};
pub use activity_data::ActivityData;
pub use activity_type::ActivityType;
