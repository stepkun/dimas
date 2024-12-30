// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// we need alloc
#[doc(hidden)]
extern crate alloc;

// see: https://robmosys.eu/wiki/start

// modules
pub mod behavior;
pub mod blackboard;
pub mod macros;
pub mod port;
pub mod utils;

// flatten:
