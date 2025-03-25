// Copyright © 2025 Stephan Kunz

//! Execution for Scripting of `DiMAS`
//!

mod chunk;
mod error;
pub mod opcodes;
pub mod values;
mod vm;

// flatten
pub use chunk::Chunk;
pub use vm::VM;
