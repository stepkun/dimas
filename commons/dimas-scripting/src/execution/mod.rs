// Copyright © 2025 Stephan Kunz

//! Execution for Scripting of `DiMAS`
//!

mod chunk;
pub mod error;
pub mod opcodes;
pub mod values;
mod vm;

// flatten
pub use chunk::Chunk;
pub use error::Error;
pub use vm::VM;
