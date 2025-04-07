// Copyright © 2025 Stephan Kunz

//! Virtual Machine for scripting of `DiMAS`
//!

mod chunk;
pub mod error;
pub mod op_code;
mod vm;

// flatten
pub use chunk::Chunk;
pub use error::Error;
pub use vm::VM;
