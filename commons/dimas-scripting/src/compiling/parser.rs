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

use alloc::{borrow::ToOwned, boxed::Box, string::ToString, sync::Arc};
use hashbrown::HashMap;

use crate::{
	Lexer,
	execution::{
		Chunk,
		opcodes::{OP_POP, OP_PRINT, OP_RETURN},
	},
};

use super::{
	error::Error,
	parselets::{
		BinaryParselet, Expression, GroupingParselet, InfixParselet, LiteralParselet,
		LogicParselet, PrefixParselet, UnaryParselet, ValueParselet, VariableParselet,
	},
	precedence::Precedence,
	token::{Token, TokenKind},
};

/// Parser
pub struct Parser<'a> {
	lexer: Lexer<'a>,
	prefix_parselets: HashMap<TokenKind, Arc<dyn PrefixParselet>>,
	infix_parselets: HashMap<TokenKind, Arc<dyn InfixParselet>>,
	/// current handled Token
	current: Token,
	/// preview on next Token
	next: Token,
}

impl<'a> Parser<'a> {
	/// Create a Parser with all the necessary ingredients
	#[must_use]
	pub fn new(source_code: &'a str) -> Self {
		let mut parser = Self {
			lexer: Lexer::new(source_code),
			prefix_parselets: HashMap::default(),
			infix_parselets: HashMap::default(),
			current: Token::none(),
			next: Token::none(),
		};

		// Register the parselets for the grammar
		parser.infix_parselets.insert(
			TokenKind::And,
			Arc::from(LogicParselet::new(Precedence::And)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Bang, Arc::from(UnaryParselet));
		parser.infix_parselets.insert(
			TokenKind::BangEqual,
			Arc::from(BinaryParselet::new(Precedence::Equality)),
		);
		parser.infix_parselets.insert(
			TokenKind::EqualEqual,
			Arc::from(BinaryParselet::new(Precedence::Equality)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::False, Arc::from(LiteralParselet));
		parser.infix_parselets.insert(
			TokenKind::Greater,
			Arc::from(BinaryParselet::new(Precedence::Comparison)),
		);
		parser.infix_parselets.insert(
			TokenKind::GreaterEqual,
			Arc::from(BinaryParselet::new(Precedence::Equality)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::HexNumber, Arc::from(ValueParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Ident, Arc::from(VariableParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::LeftParen, Arc::from(GroupingParselet));
		parser.infix_parselets.insert(
			TokenKind::Less,
			Arc::from(BinaryParselet::new(Precedence::Comparison)),
		);
		parser.infix_parselets.insert(
			TokenKind::LessEqual,
			Arc::from(BinaryParselet::new(Precedence::Equality)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Minus, Arc::from(UnaryParselet));
		parser.infix_parselets.insert(
			TokenKind::Minus,
			Arc::from(BinaryParselet::new(Precedence::Term)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Nil, Arc::from(LiteralParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Number, Arc::from(ValueParselet));
		parser
			.infix_parselets
			.insert(TokenKind::Or, Arc::from(LogicParselet::new(Precedence::Or)));
		parser
			.prefix_parselets
			.insert(TokenKind::Plus, Arc::from(UnaryParselet));
		parser.infix_parselets.insert(
			TokenKind::Plus,
			Arc::from(BinaryParselet::new(Precedence::Term)),
		);
		parser.infix_parselets.insert(
			TokenKind::Slash,
			Arc::from(BinaryParselet::new(Precedence::Factor)),
		);
		parser.infix_parselets.insert(
			TokenKind::Star,
			Arc::from(BinaryParselet::new(Precedence::Factor)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::String, Arc::from(ValueParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Tilde, Arc::from(UnaryParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::True, Arc::from(LiteralParselet));

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
		while !self.check_next(TokenKind::None) {
			//std::println!("{}", self.current.kind);
			// in case of error try to synchronize to next statement
			if let Err(error) = self.statement(&mut chunk) {
				std::println!("{error}");
				while !(self.check_next(TokenKind::Semicolon)
					|| self.check_next(TokenKind::Print)
					|| self.check_next(TokenKind::None))
				{
					self.advance()?;
				}
			}
		}

		// end compiler
		self.emit_byte(OP_RETURN, &mut chunk);
		Ok(chunk)
	}

	pub(crate) fn current(&self) -> Token {
		self.current.clone()
	}

	pub(crate) fn next(&self) -> Token {
		self.next.clone()
	}

	/// Advance to the next token
	/// # Errors
	/// passthrough of [`Lexer`] errors
	pub(crate) fn advance(&mut self) -> Result<(), Error> {
		self.current = self.next.clone();
		let tmp = self.lexer.next();
		if let Some(token) = tmp {
			// passthrough of lexer errors
			self.next = token?;
		} else {
			self.next = Token::none();
		}
		//std::println!("{}", self.current.kind);
		Ok(())
	}

	/// Consume the next token if it has the expected kind
	/// # Errors
	/// if next token does not have the expected kind
	pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<(), Error> {
		if self.next.kind == expected {
			self.advance()
		} else {
			Err(Error::ExpectedToken(
				expected.to_string(),
				self.next.kind.to_string(),
				self.next.line,
			))
		}
	}

	/// Check next token whether it has given kind
	pub(crate) fn check_next(&mut self, kind: TokenKind) -> bool {
		self.next.kind == kind
	}

	/// Check next token whether it has given kind
	pub(crate) fn match_next(&mut self, kind: TokenKind) -> bool {
		self.next.kind == kind && self.advance().is_ok()
	}

	pub(crate) fn emit_byte(&self, byte: u8, chunk: &mut Chunk) {
		chunk.write(byte, self.current.line);
	}

	pub(crate) fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
		chunk.write(byte1, self.current.line);
		chunk.write(byte2, self.current.line);
	}

	pub(crate) fn emit_jump(&self, instruction: u8, chunk: &mut Chunk) -> usize {
		chunk.write(instruction, self.current.line);
		let target_pos = chunk.code().len();
		// the dummy address bytes
		chunk.write(0x00, self.current.line);
		chunk.write(0x00, self.current.line);
		target_pos
	}

	#[allow(clippy::cast_possible_truncation)]
	pub(crate) fn patch_jump(&self, patch_pos: usize, chunk: &mut Chunk) {
		let target = chunk.code().len();
		let byte1 = (target >> 8) as u8;
		let byte2 = target as u8;
		chunk.patch(byte1, patch_pos);
		chunk.patch(byte2, (patch_pos + 1));
	}

	pub(crate) fn statement(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
		if self.next.kind == TokenKind::Print {
			self.advance()?;
			self.expression(chunk)?;
			self.consume(TokenKind::Semicolon)?;
			self.emit_byte(OP_PRINT, chunk);
		} else {
			self.expression(chunk)?;
			self.consume(TokenKind::Semicolon)?;
			//self.emit_byte(OP_POP, chunk);
		}
		Ok(())
	}

	pub(crate) fn expression(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
		self.with_precedence(Precedence::Assignment, chunk)
	}

	pub(crate) fn with_precedence(
		&mut self,
		precedence: Precedence,
		chunk: &mut Chunk,
	) -> Result<(), Error> {
		self.advance()?;

		let token = self.current();
		let prefix_opt = self.prefix_parselets.get(&token.kind);
		if prefix_opt.is_none() {
			return Err(Error::ExpressionExpected(token.line));
		}
		let prefix_parselet = prefix_opt.expect("should not fail").clone();
		prefix_parselet.parse(self, chunk, token)?;

		while precedence <= self.get_precedence() {
			self.advance()?;
			let token = self.current();
			let infix_opt = self.infix_parselets.get(&token.kind);
			if let Some(infix) = infix_opt {
				infix.clone().parse(self, chunk, token)?;
			} else {
				let prefix_opt = self.prefix_parselets.get(&token.kind);
				match infix_opt {
					Some(prefix) => prefix.clone().parse(self, chunk, token)?,
					None => {
						break;
					}
				}
			}
		}

		Ok(())
	}

	fn get_precedence(&self) -> Precedence {
		let token = self.next();
		if let Some(parselet) = self.infix_parselets.get(&token.kind) {
			return parselet.get_precedence();
		}
		Precedence::None
	}
}
