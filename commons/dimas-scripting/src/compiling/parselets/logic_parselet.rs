// Copyright © 2025 Stephan Kunz

//! `LogicParselet` for `Dimas` scripting analyzes and handles logical expressions
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

use super::InfixParselet;

pub struct LogicParselet {
	precedence: Precedence,
}

impl LogicParselet {
	pub const fn new(precedence: Precedence) -> Self {
		Self { precedence }
	}
}

impl InfixParselet for LogicParselet {
	fn parse(&self, lexer: &mut Lexer, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		// The bitwise logic does not return a boolean result but an integer
		// and resembles therefore more how arithmetic operations work.
		// The QMark Colon expression is special again.
		let kind = parser.current().kind;
		match kind {
			TokenKind::Ampersand => {
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OpCode::BitwiseAnd as u8, chunk);
				Ok(())
			}
			TokenKind::And => {
				let target_pos = parser.emit_jump(OpCode::JmpIfFalse as u8, chunk);
				parser.emit_byte(OpCode::Pop as u8, chunk);
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				Parser::patch_jump(target_pos, chunk);
				Ok(())
			}
			TokenKind::Caret => {
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OpCode::BitwiseXor as u8, chunk);
				Ok(())
			}
			TokenKind::Or => {
				let target_pos = parser.emit_jump(OpCode::JmpIfTrue as u8, chunk);
				parser.emit_byte(OpCode::Pop as u8, chunk);
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				Parser::patch_jump(target_pos, chunk);
				Ok(())
			}
			TokenKind::Pipe => {
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OpCode::BitwiseOr as u8, chunk);
				Ok(())
			}
			TokenKind::QMark => {
				let else_pos = parser.emit_jump(OpCode::JmpIfFalse as u8, chunk);
				// remove the decision value
				parser.emit_byte(OpCode::Pop as u8, chunk);
				// run the "true" expression
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				let end_pos = parser.emit_jump(OpCode::Jmp as u8, chunk);
				Parser::patch_jump(else_pos, chunk);
				// consume the ':'
				parser.consume(lexer, TokenKind::Colon)?;
				// remove the decision value
				parser.emit_byte(OpCode::Pop as u8, chunk);
				// run the "false" expression
				parser.with_precedence(lexer, self.precedence.next_higher(), chunk)?;
				Parser::patch_jump(end_pos, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().into(), line!())),
		}
	}

	fn get_precedence(&self) -> Precedence {
		self.precedence
	}
}
