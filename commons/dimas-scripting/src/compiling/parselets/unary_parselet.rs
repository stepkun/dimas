// Copyright © 2025 Stephan Kunz

//! `UnaryParselet` for `Dimas`scripting
//!

use alloc::string::ToString;

use crate::{
	Parser,
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_BITWISE_NOT, OP_NEGATE, OP_NOT},
	},
};

use super::PrefixParselet;

pub struct UnaryParselet;

impl PrefixParselet for UnaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let token = parser.current();
		// there must be a current token
		if parser.next().kind == TokenKind::None {
			return Err(Error::ExpressionExpected(parser.next().line));
		}
		// compile the operand
		parser.with_precedence(Precedence::Unary, chunk)?;
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
			TokenKind::Tilde => {
				// add the binary not
				parser.emit_byte(OP_BITWISE_NOT, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}
}
