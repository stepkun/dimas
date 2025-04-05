// Copyright © 2024 Stephan Kunz

//! Port of `DiMAS`

pub mod macros;
#[allow(clippy::module_inception)]
mod port;

// flatten
pub use port::*;
