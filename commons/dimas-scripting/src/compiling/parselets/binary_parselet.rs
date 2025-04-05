// Copyright © 2025 Stephan Kunz

//! `BinaryParselet` for `Dimas` scripting analyzses and handles the binary expressions
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
		op_code::OpCode,
	},
};

use super::InfixParselet;

pub struct BinaryParselet {
	precedence: Precedence,
}

impl BinaryParselet {
	pub const fn new(precedence: Precedence) -> Self {
		Self { precedence }
	}
}

impl InfixParselet for BinaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let kind = parser.current().kind;
		parser.with_precedence(self.precedence.next_higher(), chunk)?;
		match kind {
			TokenKind::BangEqual => {
				parser.emit_bytes(OpCode::Equal as u8, OpCode::Not as u8, chunk);
				Ok(())
			}
			TokenKind::EqualEqual => {
				parser.emit_byte(OpCode::Equal as u8, chunk);
				Ok(())
			}
			TokenKind::Greater => {
				parser.emit_byte(OpCode::Greater as u8, chunk);
				Ok(())
			}
			TokenKind::GreaterEqual => {
				parser.emit_bytes(OpCode::Less as u8, OpCode::Not as u8, chunk);
				Ok(())
			}
			TokenKind::Less => {
				parser.emit_byte(OpCode::Less as u8, chunk);
				Ok(())
			}
			TokenKind::LessEqual => {
				parser.emit_bytes(OpCode::Greater as u8, OpCode::Not as u8, chunk);
				Ok(())
			}
			TokenKind::Plus => {
				parser.emit_byte(OpCode::Add as u8, chunk);
				Ok(())
			}
			TokenKind::Minus => {
				parser.emit_byte(OpCode::Subtract as u8, chunk);
				Ok(())
			}
			TokenKind::Star => {
				parser.emit_byte(OpCode::Multiply as u8, chunk);
				Ok(())
			}
			TokenKind::Slash => {
				parser.emit_byte(OpCode::Divide as u8, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}

	fn get_precedence(&self) -> Precedence {
		self.precedence
	}
}
