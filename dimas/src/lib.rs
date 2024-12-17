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

// ---------------------------------------------------

pub mod agent_old;
pub mod builder_old;
pub mod context_old;
pub mod error_old;
mod utils_old;

// macro reexport
pub use dimas_macros::{agent, main};

// mostly needed stuff
pub mod prelude_old;

// ---------------------------------------------------
