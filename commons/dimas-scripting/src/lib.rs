// Copyright © 2025 Stephan Kunz
#![no_std]

//! A scripting language for `DiMAS`
//!
//! This scripting language follows the pattern of clox as described in Part III of [crafting interpreters](https://craftinginterpreters.com/)
//!
//! Definition of the grammer following this [notation](https://craftinginterpreters.com/representing-code.html#rules-for-grammars)
//!
//! ```no-test
//! script      → statement* EoF ;
//! statement   → expression ";" ;
//! expression  → assignment ;
//! assignment  → IDENTIFIER ":=" assignment | IDENTIFIER "=" assignment | logic_or ;
//! logic_or    → logic_and ( "||" logic_and )* ;
//! logic_and   → equality ( "&&" equality )* ;
//! equality    → comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//! term        → factor ( ( "-" | "+" ) factor )* ;
//! factor      → unary ( ( "/" | "*" ) unary )* ;
//! unary       → ( "!" | "-" ) unary | primary ;
//! primary     → "true" | "false" | NUMBER | HEXNUMBER | STRING | IDENTIFIER | "(" expression ")" ;
//!
//! NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
//! HEXNUMBER   → DIGIT+ | "a" ... "f"+ | "A" ... "F"+ ;
//! STRING      → "\'" <any char except "\'">* "\'" ;
//! IDENTIFIER  → ALPHA ( ALPHA | DIGIT )* ;
//! ALPHA       → "a" ... "z" | "A" ... "Z" | "_" ;
//! DIGIT       → "0" ... "9" ;
//!
//! TODO'S:
//! ternary     → condition "?" term ":" term ;
//! binary_xor  → binary_or ( "|" binary_or )* ;
//! binary_or   → binary_and ( "|" binary_and )* ;
//! binary_and  → equality ( "&" equality )* ;
//! c_assgnmnt  → IDENTIFIER ( "+=" | "-=" | "*="" | "/=" ) term ;
//! ```
//!

#[doc(hidden)]
extern crate alloc;

mod compiling;
mod execution;

use alloc::string::{String, ToString};
// flatten
pub use compiling::Parser;
pub use compiling::{Lexer, TokenKind};
use execution::Error;
pub use execution::VM;

use execution::values::Value;
use hashbrown::HashMap;
use parking_lot::RwLock;

/// The trait for providing an [`Environment`] to a [`VM`] that stores variables persistently and externally available.
pub trait Environment: Send + Sync {
	/// Define the variable with `name` to `value`.
	/// It has to be created if it does not already exist.
	fn define(&self, name: &str, value: Value);
	/// Get a variable by name
	/// # Errors
	/// if the variable does not exist
	fn get(&self, name: &str) -> Result<Value, Error>;
	/// Set the variable with `name` to `value`.
	/// # Errors
	/// if variable does not exist.
	fn set(&self, name: &str, value: Value) -> Result<(), Error>;
}

/// A default Environment for testing purpose end the REPL
#[derive(Default)]
pub struct DefaultEnvironment {
	storage: RwLock<HashMap<String, Value>>,
}

impl Environment for DefaultEnvironment {
	fn define(&self, name: &str, value: Value) {
		self.storage
			.write()
			.insert(name.to_string(), value);
	}

	fn get(&self, name: &str) -> Result<Value, Error> {
		self.storage
			.read()
			.get(name)
			.map_or(Err(Error::GlobalNotDefined), |value| Ok(*value))
	}

	fn set(&self, name: &str, value: Value) -> Result<(), Error> {
		if self.storage.read().contains_key(name) {
			self.storage
				.write()
				.insert(name.to_string(), value);
			Ok(())
		} else {
			Err(Error::GlobalNotDefined)
		}
	}
}
