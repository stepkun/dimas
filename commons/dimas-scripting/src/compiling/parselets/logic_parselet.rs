// Copyright © 2025 Stephan Kunz

//! `GroupingParselet` for `Dimas`scripting
//!

use alloc::string::ToString;

use crate::{
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	}, execution::{
		opcodes::{
			OP_BITWISE_AND, OP_BITWISE_OR, OP_BITWISE_XOR, OP_JMP, OP_JMP_IF_FALSE, OP_JMP_IF_TRUE, OP_POP
		}, Chunk
	}, Parser
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
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		// The bitwise logic does not return a boolean result but an integer
		// and resembles therefore more how arithmetic operations work.
		// The QMark Colon expression is special again.
		let kind = parser.current().kind;
		match kind {
			TokenKind::Ampersand => {
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OP_BITWISE_AND, chunk);
				Ok(())
			}
			TokenKind::And => {
				let target_pos = parser.emit_jump(OP_JMP_IF_FALSE, chunk);
				parser.emit_byte(OP_POP, chunk);
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.patch_jump(target_pos, chunk);
				Ok(())
			}
			TokenKind::Caret => {
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OP_BITWISE_XOR, chunk);
				Ok(())
			}
			TokenKind::Or => {
				let target_pos = parser.emit_jump(OP_JMP_IF_TRUE, chunk);
				parser.emit_byte(OP_POP, chunk);
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.patch_jump(target_pos, chunk);
				Ok(())
			}
			TokenKind::Pipe => {
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.emit_byte(OP_BITWISE_OR, chunk);
				Ok(())
			}
			TokenKind::QMark => {
				let else_pos = parser.emit_jump(OP_JMP_IF_FALSE, chunk);
				// remove the decision value
				parser.emit_byte(OP_POP, chunk);
				// run the "true" expression
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				let end_pos = parser.emit_jump(OP_JMP, chunk);
				parser.patch_jump(else_pos, chunk);
				// consume the ':'
				parser.consume(TokenKind::Colon)?;
				// remove the decision value
				parser.emit_byte(OP_POP, chunk);
				// run the "false" expression
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.patch_jump(end_pos, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}

	fn get_precedence(&self) -> Precedence {
		self.precedence
	}
}
