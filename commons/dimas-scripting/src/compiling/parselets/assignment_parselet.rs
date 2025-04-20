// Copyright © 2025 Stephan Kunz

//! `AssignmentParselet` for `Dimas` scripting handles all kinds of assignments
//!

// region:   	--- modules
use crate::{
	Lexer, Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::{Chunk, ScriptingValue, op_code::OpCode},
};

use super::PrefixParselet;
// endregion:   --- modules

pub struct AssignmentParselet;

impl PrefixParselet for AssignmentParselet {
	fn parse(
		&self,
		lexer: &mut Lexer,
		parser: &mut Parser,
		chunk: &mut Chunk,
		token: Token,
	) -> Result<(), Error> {
		match parser.next().kind {
			TokenKind::ColonEqual => {
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::DefineExternal as u8, name, chunk);
			}
			TokenKind::PlusEqual => {
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::GetExternal as u8, name, chunk);
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				parser.emit_byte(OpCode::Add as u8, chunk);
				parser.emit_bytes(OpCode::SetExternal as u8, name, chunk);
			}
			TokenKind::MinusEqual => {
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::GetExternal as u8, name, chunk);
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				parser.emit_byte(OpCode::Subtract as u8, chunk);
				parser.emit_bytes(OpCode::SetExternal as u8, name, chunk);
			}
			TokenKind::StarEqual => {
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::GetExternal as u8, name, chunk);
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				parser.emit_byte(OpCode::Multiply as u8, chunk);
				parser.emit_bytes(OpCode::SetExternal as u8, name, chunk);
			}
			TokenKind::SlashEqual => {
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::GetExternal as u8, name, chunk);
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				parser.emit_byte(OpCode::Divide as u8, chunk);
				parser.emit_bytes(OpCode::SetExternal as u8, name, chunk);
			}
			TokenKind::Equal => {
				parser.advance(lexer)?;
				parser.expression(lexer, chunk)?;
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::SetExternal as u8, name, chunk);
			}
			_ => {
				let name = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::GetExternal as u8, name, chunk);
			}
		}
		Ok(())
	}
}
