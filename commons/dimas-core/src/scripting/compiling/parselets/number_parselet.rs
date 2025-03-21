// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `NumberParselet` for `Dimas`scripting
//!

use alloc::{boxed::Box, string::ToString};

use crate::scripting::{
	Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::{Chunk, opcodes::OP_CONSTANT, values::Value},
};

use super::{Expression, PrefixParselet};

pub struct NumberParselet;

impl PrefixParselet for NumberParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		match token.kind {
			TokenKind::Number => {
				let double: f64 = match token.origin.parse() {
					Ok(n) => n,
					Err(e) => {
						return Err(Error::ParseNumber(token.origin.to_string(), token.line));
					}
				};

				let offset = chunk.add_constant(Value::from_double(double));
				if offset == u8::MAX {
					return Err(Error::ToManyValues);
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
				let offset = chunk.add_constant(Value::from_integer(value));
				if offset == u8::MAX {
					return Err(Error::ToManyValues);
				}
				parser.emit_bytes(OP_CONSTANT, offset, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}
}
