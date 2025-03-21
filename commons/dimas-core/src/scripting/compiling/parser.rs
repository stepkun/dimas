// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unused_self)]
#![allow(clippy::needless_pass_by_ref_mut)]

//! Parser for `DiMAS` scripting implemented as a [Pratt-Parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
//! You should also read th earticel by [Robert Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
//!
//! Implementation is inspired by
//! - Jon Gjengsets [video](https://www.youtube.com/watch?v=mNOLaw-_Buc) & [example](https://github.com/jonhoo/lox/blob/master/src/parse.rs)
//! - Jürgen Wurzers implementation of [Bantam](https://github.com/jwurzer/bantam-rust/blob/master/src/bantam/bantam_parser.rs)
//!

extern crate std;

use alloc::{borrow::ToOwned, boxed::Box, rc::Rc};
use hashbrown::HashMap;

use crate::scripting::{
	Lexer,
	execution::{Chunk, opcodes::OP_RETURN},
};

use super::{
	error::Error,
	parselets::{
		BinaryParselet, Expression, GroupingParselet, InfixParselet, LiteralParselet,
		ValueParselet, PrefixParselet, UnaryParselet,
	},
	precedence::{ASSIGNMENT, COMPARISON, EQUALITY, FACTOR, NONE, Precedence, TERM, UNARY},
	token::{Token, TokenKind},
};

/// Parser
pub struct Parser<'a> {
	lexer: Lexer<'a>,
	prefix_parselets: HashMap<TokenKind, Rc<dyn PrefixParselet>>,
	infix_parselets: HashMap<TokenKind, Rc<dyn InfixParselet>>,
	previous: Token,
	current: Token,
}

impl<'a> Parser<'a> {
	/// Create a Parser with all the necessary ingredients
	#[must_use]
	pub fn new(source_code: &'a str) -> Self {
		let mut parser = Self {
			lexer: Lexer::new(source_code),
			prefix_parselets: HashMap::default(),
			infix_parselets: HashMap::default(),
			previous: Token::none(),
			current: Token::none(),
		};

		// Register the parselets for the grammar
		parser
			.prefix_parselets
			.insert(TokenKind::Bang, Rc::from(UnaryParselet::new(NONE)));
		parser.infix_parselets.insert(
			TokenKind::BangEqual,
			Rc::from(BinaryParselet::new(EQUALITY, false)),
		);
		parser.infix_parselets.insert(
			TokenKind::EqualEqual,
			Rc::from(BinaryParselet::new(EQUALITY, false)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::False, Rc::from(LiteralParselet));
		parser.infix_parselets.insert(
			TokenKind::Greater,
			Rc::from(BinaryParselet::new(COMPARISON, false)),
		);
		parser.infix_parselets.insert(
			TokenKind::GreaterEqual,
			Rc::from(BinaryParselet::new(EQUALITY, false)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::LeftParen, Rc::from(GroupingParselet));
		parser.infix_parselets.insert(
			TokenKind::Less,
			Rc::from(BinaryParselet::new(COMPARISON, false)),
		);
		parser.infix_parselets.insert(
			TokenKind::LessEqual,
			Rc::from(BinaryParselet::new(EQUALITY, false)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Minus, Rc::from(UnaryParselet::new(UNARY)));
		parser
			.infix_parselets
			.insert(TokenKind::Minus, Rc::from(BinaryParselet::new(TERM, false)));
		parser
			.prefix_parselets
			.insert(TokenKind::Nil, Rc::from(LiteralParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Number, Rc::from(ValueParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Plus, Rc::from(UnaryParselet::new(UNARY)));
		parser
			.infix_parselets
			.insert(TokenKind::Plus, Rc::from(BinaryParselet::new(TERM, false)));
		parser.infix_parselets.insert(
			TokenKind::Slash,
			Rc::from(BinaryParselet::new(FACTOR, false)),
		);
		parser.infix_parselets.insert(
			TokenKind::Star,
			Rc::from(BinaryParselet::new(FACTOR, false)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::String, Rc::from(ValueParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::True, Rc::from(LiteralParselet));

		// return the prepared parser
		parser
	}

	/// Create a bytecode [`Chunk`] from source
	/// # Errors
	/// - passes [`Lexer`] errors through
	/// - if it could not create a proper [`Chunk`]
	pub fn parse(&mut self) -> Result<Chunk, Error> {
		let mut chunk = Chunk::default();

		self.advance()?;
		self.expression(&mut chunk);
		self.emit_byte(OP_RETURN, &mut chunk);
		Ok(chunk)
	}

	pub(crate) fn previous(&self) -> Token {
		self.previous.clone()
	}

	pub(crate) fn current(&self) -> Token {
		self.current.clone()
	}

	/// Advance to the next token
	pub(crate) fn advance(&mut self) -> Result<(), Error> {
		self.previous = self.current();
		let tmp = self.lexer.next();
		if let Some(token) = tmp {
			self.current = token?;
		} else {
			self.current = Token::none();
		}
		Ok(())
	}

	/// Advance to next token if it has given kind
	pub(crate) fn advance_if(&mut self, kind: TokenKind) -> Result<(), Error> {
		if self.current.kind != kind {
			return Err(Error::UnexpectedToken);
		}
		self.advance()
	}

	pub(crate) fn emit_byte(&self, byte: u8, chunk: &mut Chunk) {
		chunk.write(byte, self.previous.line);
	}

	pub(crate) fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
		chunk.write(byte1, self.previous.line);
		chunk.write(byte2, self.previous.line);
	}

	pub(crate) fn expression(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
		self.with_precedence(ASSIGNMENT, chunk)
	}

	pub(crate) fn with_precedence(
		&mut self,
		precedence: Precedence,
		chunk: &mut Chunk,
	) -> Result<(), Error> {
		self.advance()?;

		let token = self.previous();
		let prefix_opt = self.prefix_parselets.get(&token.kind);
		if prefix_opt.is_none() {
			return Err(Error::ExpressionExpected);
		}
		let prefix = prefix_opt.expect("should not fail").clone();
		prefix.parse(self, chunk, token)?;

		while precedence <= self.get_precedence() {
			self.advance()?;
			let token = self.previous();
			let infix_opt = self.infix_parselets.get(&token.kind);
			match infix_opt {
				Some(infix) => infix.clone().parse(self, chunk, token)?,
				None => {
					break;
				}
			}
		}

		Ok(())
	}

	fn get_precedence(&self) -> Precedence {
		let token = self.current();
		if let Some(parselet) = self.infix_parselets.get(&token.kind) {
			return parselet.get_precedence();
		}
		NONE
	}
}
