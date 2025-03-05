// Copyright © 2025 Stephan Kunz

//! Scripting of `DiMAS`

pub mod error;
mod lex;
mod parse;
mod token;

// flatten
pub use lex::Lexer;
pub use parse::Parser;
pub use token::*;
