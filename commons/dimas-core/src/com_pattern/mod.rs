// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
#![allow(unused)]

//! Communication pattern traits & structs
//!

mod error;
mod communicator;
mod observable;
mod observer;
mod publisher;
mod querier;
mod queryable;
mod subscriber;

// flatten
pub use communicator::CommunicatorFactory;
