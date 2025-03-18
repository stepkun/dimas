// Copyright © 2025 Stephan Kunz

//! Execution for Scripting of `DiMAS`
//!

mod lexer;
mod token;

// flatten
pub use lexer::Lexer;
pub use token::{Token, TokenKind};
