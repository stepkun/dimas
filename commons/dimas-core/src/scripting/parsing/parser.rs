// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Parser for `DiMAS` scripting implemented as a [Pratt-Parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
//! You should also read th earticel by [Robert Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
//!
//! Implementation is heavily inspired by
//! - Jon Gjengsets [video](https://www.youtube.com/watch?v=mNOLaw-_Buc) & [example](https://github.com/jonhoo/lox/blob/master/src/parse.rs)
//! - Jürgen Wurzers implementation of [Bantam](https://github.com/jwurzer/bantam-rust/blob/master/src/bantam/bantam_parser.rs)
//!

extern crate std;

use alloc::rc::Rc;
use hashbrown::HashMap;

use crate::scripting::{Chunk, Lexer, TokenKind, error::Error};

use super::parselets::{InfixParselet, PrefixParselet};

/// Parser
pub struct Parser<'a> {
	whole: &'a str,
	lexer: Lexer<'a>,
	prefix_parselets: HashMap<TokenKind, Rc<dyn PrefixParselet>>,
	infix_parselets: HashMap<TokenKind, Rc<dyn InfixParselet>>,
}

impl<'a> Parser<'a> {
	/// Create a Parser
	#[must_use]
	pub fn new(source_code: &'a str) -> Self {
		let parser = Self {
			whole: source_code,
			lexer: Lexer::new(source_code),
			prefix_parselets: HashMap::default(),
			infix_parselets: HashMap::default(),
		};

		// Register the parselets for the grammar

		parser
	}

	/// Create a bytecode [`Chunk`] from source
	/// # Errors
	/// - passes [`Lexer`] errors through
	/// - if it could not create a proper [`Chunk`]
	pub fn parse(&mut self) -> Result<Chunk, Error> {
		let chunk = Chunk::default();
		for token in self.lexer.by_ref() {
			match token {
				Ok(token) => {
					std::println!("{:?}", token.kind);
				}
				Err(err) => {
					return Err(err);
				}
			}
		}
		Ok(chunk)
	}
}
