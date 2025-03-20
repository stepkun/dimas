// Copyright © 2025 Stephan Kunz

//! Bytecode compiler for `DiMAS` scripting
//!

mod error;
mod lexer;
mod parselets;
#[allow(clippy::module_inception)]
mod parser;
mod precedence;
mod token;

// flatten
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::TokenKind;
