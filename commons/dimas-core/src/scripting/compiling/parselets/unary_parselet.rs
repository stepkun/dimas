// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `UnaryParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	Parser,
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_CONSTANT, OP_NEGATE, OP_NOT},
	},
};

use super::{Expression, PrefixParselet};

pub struct UnaryParselet {
	precedence: Precedence,
}

impl UnaryParselet {
	pub const fn new(precedence: Precedence) -> Self {
		Self { precedence }
	}
}

impl PrefixParselet for UnaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		let token = parser.current();
		// there must be a current token
		if parser.next().kind == TokenKind::None {
			return Err(Error::ExpressionExpected(parser.next().line));
		}
		// compile the operand
		parser.with_precedence(self.precedence, chunk)?;
		match token.kind {
			TokenKind::Bang => {
				// add the logical not
				parser.emit_byte(OP_NOT, chunk);
				Ok(())
			}
			TokenKind::Minus => {
				// add the negation
				parser.emit_byte(OP_NEGATE, chunk);
				Ok(())
			}
			TokenKind::Plus => {
				// do nothing
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}
}
