// Copyright © 2025 Stephan Kunz

//! `UnaryParselet` for `Dimas` scripting analyzes and handles the prefix expressions
//!

use crate::{
	Error,
	compiling::{
		Lexer, Parser,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{Chunk, op_code::OpCode},
};

use super::PrefixParselet;

pub struct UnaryParselet;

impl PrefixParselet for UnaryParselet {
	fn parse(&self, lexer: &mut Lexer, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let token = parser.current();
		// there must be a current token
		if parser.next().kind == TokenKind::None {
			return Err(Error::ExpressionExpected(parser.next().line));
		}
		// compile the operand
		parser.with_precedence(lexer, Precedence::Unary, chunk)?;
		match token.kind {
			TokenKind::Bang => {
				// add the logical not
				parser.emit_byte(OpCode::Not as u8, chunk);
				Ok(())
			}
			TokenKind::Minus => {
				// add the negation
				parser.emit_byte(OpCode::Negate as u8, chunk);
				Ok(())
			}
			TokenKind::Plus => {
				// do nothing
				Ok(())
			}
			TokenKind::Tilde => {
				// add the binary not
				parser.emit_byte(OpCode::BitwiseNot as u8, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().into(), line!())),
		}
	}
}
