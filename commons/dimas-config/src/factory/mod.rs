// Copyright © 2024 Stephan Kunz

//! [`BehaviorTree`] factory of `DiMAS`

// we need alloc
#[doc(hidden)]
extern crate alloc;

// modules
mod error;
#[allow(clippy::module_inception)]
mod factory;
mod xml_parser;

// flatten:
#[allow(clippy::module_name_repetitions)]
pub use factory::BTFactory;
pub use error::Error;
