// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core of `DiMAS`

// see: https://robmosys.eu/wiki/start

// modules
mod error;
mod utils;

// flatten:
pub use utils::init_tracing;