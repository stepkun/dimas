// Copyright © 2025 Stephan Kunz

//! Parser for `DiMAS` scripting implemented as a [Pratt-Parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
//! You should also read the articel by [Robert Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
//!
//! Implementation is inspired by
//! - Robert Nystrom [crafting interpreters - A Bytecode Virtual Machine](https://craftinginterpreters.com/a-bytecode-virtual-machine.html)
//! - Jon Gjengsets [video](https://www.youtube.com/watch?v=mNOLaw-_Buc) & [example](https://github.com/jonhoo/lox/blob/master/src/parse.rs)
//! - Jürgen Wurzers implementation of [Bantam](https://github.com/jwurzer/bantam-rust/blob/master/src/bantam/bantam_parser.rs)
//!

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{
	collections::btree_map::BTreeMap,
	string::{String, ToString},
	sync::Arc,
};

use crate::{
	Error,
	compiling::Lexer,
	execution::{Chunk, op_code::OpCode},
};

use super::{
	parselets::{
		AssignmentParselet, BinaryParselet, GroupingParselet, InfixParselet, LiteralParselet,
		LogicParselet, PrefixParselet, UnaryParselet, ValueParselet,
	},
	precedence::Precedence,
	token::{Token, TokenKind},
};
// endregion:	--- modules

// region:		--- Parser
/// Parser
pub struct Parser {
	prefix_parselets: BTreeMap<TokenKind, Arc<dyn PrefixParselet>>,
	infix_parselets: BTreeMap<TokenKind, Arc<dyn InfixParselet>>,
	/// current handled Token
	current: Token,
	/// preview on next Token
	next: Token,
}

impl core::fmt::Debug for Parser {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Parser")
			// .field("lexer", &self.lexer)
			// .field("prefix_parselets", &self.prefix_parselets)
			// .field("infix_parselets", &self.infix_parselets)
			.field("current", &self.current)
			.field("next", &self.next)
			.finish_non_exhaustive()
	}
}

impl Default for Parser {
	fn default() -> Self {
		Self::new()
	}
}

impl Parser {
	/// Create a Parser with all the necessary ingredients
	#[must_use]
	#[allow(clippy::too_many_lines)]
	pub fn new() -> Self {
		let mut parser = Self {
			prefix_parselets: BTreeMap::default(),
			infix_parselets: BTreeMap::default(),
			current: Token::none(),
			next: Token::none(),
		};

		// Register the parselets for the grammar
		parser.infix_parselets.insert(
			TokenKind::Ampersand,
			Arc::from(LogicParselet::new(Precedence::BitAnd)),
		);
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
			TokenKind::Caret,
			Arc::from(LogicParselet::new(Precedence::BitXor)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Enum, Arc::from(ValueParselet));
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
			.insert(TokenKind::IntNumber, Arc::from(ValueParselet));
		parser
			.prefix_parselets
			.insert(TokenKind::Ident, Arc::from(AssignmentParselet));
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
			.insert(TokenKind::FloatNumber, Arc::from(ValueParselet));
		parser
			.infix_parselets
			.insert(TokenKind::Or, Arc::from(LogicParselet::new(Precedence::Or)));
		parser.infix_parselets.insert(
			TokenKind::Pipe,
			Arc::from(LogicParselet::new(Precedence::BitOr)),
		);
		parser
			.prefix_parselets
			.insert(TokenKind::Plus, Arc::from(UnaryParselet));
		parser.infix_parselets.insert(
			TokenKind::Plus,
			Arc::from(BinaryParselet::new(Precedence::Term)),
		);
		parser.infix_parselets.insert(
			TokenKind::QMark,
			Arc::from(LogicParselet::new(Precedence::Ternary)),
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
	pub fn parse(
		&mut self,
		enums: &BTreeMap<String, i8>,
		source_code: &str,
	) -> Result<Chunk, Error> {
		let mut chunk = Chunk::default();
		let mut lexer = Lexer::new(enums, source_code);

		self.advance(&mut lexer)?;
		while !self.check_next(TokenKind::None) {
			//std::println!("{}", self.current.kind);
			// in case of error try to synchronize to next statement
			if let Err(error) = self.statement(&mut lexer, &mut chunk) {
				#[cfg(feature = "std")]
				std::println!("{error}");
				while !(self.check_next(TokenKind::Semicolon)
					|| self.check_next(TokenKind::Print)
					|| self.check_next(TokenKind::None))
				{
					self.advance(&mut lexer)?;
				}
			}
		}

		// end compiler
		self.emit_byte(OpCode::Return as u8, &mut chunk);
		chunk.finalize();
		Ok(chunk)
	}

	pub(super) fn current(&self) -> Token {
		self.current.clone()
	}

	pub(super) fn next(&self) -> Token {
		self.next.clone()
	}

	/// Advance to the next token
	/// # Errors
	/// passthrough of [`Lexer`] errors
	pub(super) fn advance(&mut self, lexer: &mut Lexer) -> Result<(), Error> {
		self.current = self.next.clone();
		let tmp = lexer.next();
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
	pub(super) fn consume(&mut self, lexer: &mut Lexer, expected: TokenKind) -> Result<(), Error> {
		if self.next.kind == expected {
			self.advance(lexer)
		} else {
			Err(Error::ExpectedToken(
				expected.to_string().into(),
				self.next.kind.to_string().into(),
				self.next.line,
			))
		}
	}

	/// Check next token whether it has given kind
	pub(super) fn check_next(&self, kind: TokenKind) -> bool {
		self.next.kind == kind
	}

	pub(super) fn emit_byte(&self, byte: u8, chunk: &mut Chunk) {
		chunk.write(byte, self.current.line);
	}

	pub(super) fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
		chunk.write(byte1, self.current.line);
		chunk.write(byte2, self.current.line);
	}

	pub(super) fn emit_jump(&self, instruction: u8, chunk: &mut Chunk) -> usize {
		chunk.write(instruction, self.current.line);
		let target_pos = chunk.code().len();
		// the dummy address bytes
		chunk.write(0xFF, self.current.line);
		chunk.write(0xFF, self.current.line);
		target_pos
	}

	#[allow(clippy::cast_possible_truncation)]
	pub(super) fn patch_jump(patch_pos: usize, chunk: &mut Chunk) {
		let target = chunk.code().len();
		let byte1 = (target >> 8) as u8;
		let byte2 = target as u8;
		chunk.patch(byte1, patch_pos);
		chunk.patch(byte2, patch_pos + 1);
	}

	pub(super) fn statement(&mut self, lexer: &mut Lexer, chunk: &mut Chunk) -> Result<(), Error> {
		if self.next.kind == TokenKind::Print {
			self.advance(lexer)?;
			self.expression(lexer, chunk)?;
			// a statement my also be finished by EOF
			if !self.check_next(TokenKind::None) {
				self.consume(lexer, TokenKind::Semicolon)?;
			}
			self.emit_byte(OpCode::Print as u8, chunk);
		} else {
			self.expression(lexer, chunk)?;
			if !self.check_next(TokenKind::None) {
				self.consume(lexer, TokenKind::Semicolon)?;
			}
			//self.emit_byte(OP_POP, chunk);
		}
		Ok(())
	}

	pub(super) fn expression(&mut self, lexer: &mut Lexer, chunk: &mut Chunk) -> Result<(), Error> {
		self.with_precedence(lexer, Precedence::Assignment, chunk)
	}

	pub(super) fn with_precedence(
		&mut self,
		lexer: &mut Lexer,
		precedence: Precedence,
		chunk: &mut Chunk,
	) -> Result<(), Error> {
		self.advance(lexer)?;

		let token = self.current();
		let prefix_opt = self.prefix_parselets.get(&token.kind);
		if prefix_opt.is_none() {
			return Err(Error::ExpressionExpected(token.line));
		}
		let prefix_parselet = prefix_opt.expect("should not fail").clone();
		prefix_parselet.parse(lexer, self, chunk, token)?;

		while precedence <= self.get_precedence() {
			self.advance(lexer)?;
			let token = self.current();
			let infix_opt = self.infix_parselets.get(&token.kind);
			if let Some(infix) = infix_opt {
				infix.clone().parse(lexer, self, chunk, token)?;
			} else {
				let _prefix_opt = self.prefix_parselets.get(&token.kind);
				match infix_opt {
					Some(prefix) => prefix.clone().parse(lexer, self, chunk, token)?,
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
// endregion:	--- Parser
