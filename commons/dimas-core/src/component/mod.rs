// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

//! Component traits & structs
//!

mod component;
mod component_data;
mod component_type;
mod error;

// flatten
pub use component::{Component, ComponentId};
pub use component_data::ComponentData;
pub use component_type::ComponentType;
