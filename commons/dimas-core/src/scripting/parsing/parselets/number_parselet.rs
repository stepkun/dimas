// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `NumberParselet` for `Dimas`scripting
//!

use alloc::{boxed::Box, string::ToString};

use crate::scripting::{
	Chunk, Parser, TokenKind, error::Error, execution::opcodes::OP_CONSTANT, lexing::Token,
};

use super::{Expression, PrefixParselet};

pub struct NumberParselet;

impl PrefixParselet for NumberParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		match token.kind {
			TokenKind::Number => {
				let value = match token.origin.parse() {
					Ok(n) => n,
					Err(e) => {
						return Err(Error::ParseNumber(token.origin.to_string(), token.line));
					}
				};

				let offset = chunk.add_constant(value);
				if offset == u8::MAX {
					return Err(Error::ToManyNumbers);
				}
				parser.emit_bytes(OP_CONSTANT, offset, chunk);
				Ok(())
			}
			TokenKind::HexNumber => {
				// remove the '0x' before parsing
				let literal = token.origin.trim_start_matches("0x");
				let value = match i64::from_str_radix(literal, 16) {
					Ok(i) => i,
					Err(e) => {
						return Err(Error::ParseHex(literal.to_string(), token.line));
					}
				};
				let offset = chunk.add_hex_constant(value);
				if offset == u8::MAX {
					return Err(Error::ToManyHexNumbers);
				}
				parser.emit_bytes(OP_CONSTANT, offset, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}
}
