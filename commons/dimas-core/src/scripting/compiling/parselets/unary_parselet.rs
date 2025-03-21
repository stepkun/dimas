// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `UnaryParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	compiling::{
		error::Error,
		precedence::{Precedence, UNARY},
		token::{Token, TokenKind},
	}, execution::{
		opcodes::{OP_CONSTANT, OP_NEGATE, OP_NOT}, Chunk
	}, Parser
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
		let token = parser.previous();
		// there must be a current token
		if parser.current().kind == TokenKind::None {
			return Err(Error::ExpressionExpected)
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
