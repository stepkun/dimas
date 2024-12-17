// Copyright © 2024 Stephan Kunz
//! Zenoh based communication
#![allow(clippy::module_inception)]

mod subscriber;
mod zenoh;

// flatten
pub use zenoh::Zenoh;
