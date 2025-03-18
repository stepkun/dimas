// Copyright © 2025 Stephan Kunz

//! Execution for Scripting of `DiMAS`
//!

pub mod opcodes;
pub mod values;
mod vm;

// flatten
pub use vm::VM;
