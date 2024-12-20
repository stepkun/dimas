// Copyright © 2023 Stephan Kunz
#![crate_type = "lib"]
#![crate_name = "dimas"]
//#![no_panic]
#![doc = include_str!("../README.md")]

//! ## Public interface
//!
//! Typically it is sufficient to include the prelude with
//!
//! ```use dimas::prelude::*;```

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod agent;
mod com;
mod error;
mod timer;

pub use agent::Agent;

// mostly needed stuff
pub mod prelude;

// macro reexport
pub use dimas_macros::main;
