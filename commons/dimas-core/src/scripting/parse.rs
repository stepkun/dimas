// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Parser for `DiMAS` scripting

use super::Lexer;

/// Parser
pub struct Parser<'a> {
	whole: &'a str,
	lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
	/// Create a Parser
	#[must_use]
	pub const fn new(source_code: &'a str) -> Self {
		Self {
			whole: source_code,
			lexer: Lexer::new(source_code),
		}
	}
}
