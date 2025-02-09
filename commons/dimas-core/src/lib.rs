// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

#[doc(hidden)]
extern crate alloc;

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// see: https://robmosys.eu/wiki/start

// modules
pub mod behavior;
pub mod blackboard;
pub mod port;
pub mod utils;

// flatten:
