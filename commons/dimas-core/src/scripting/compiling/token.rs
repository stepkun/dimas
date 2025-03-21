// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Token for `DiMAS` scripting

use alloc::string::{String, ToString};

/// Token kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
	/// Dummy to avoid using `Option<Token>` in many places
	None,
	/// =
	Equal,
	/// :
	Colon,
	/// :=
	ColonEqual,
	/// +
	Plus,
	/// +=
	PlusEqual,
	/// -
	Minus,
	/// -=
	MinusEqual,
	/// *
	Star,
	/// *=
	StarEqual,
	/// /
	Slash,
	/// /=
	SlashEqual,
	/// ;
	Semicolon,
	/// & -> binary and
	Ampersand,
	/// | -> binary or
	Pipe,
	/// ^ -> binary xor
	Caret,
	/// ~ -> binary not
	Tilde,
	/// && -> logic and
	And,
	/// || -> logic or
	Or,
	/// ! -> logic not
	Bang,
	/// !=
	BangEqual,
	/// ==
	EqualEqual,
	/// <
	Less,
	/// <=
	LessEqual,
	/// >
	Greater,
	/// >=
	GreaterEqual,
	/// ?
	QMark,
	/// (
	LeftParen,
	/// )
	RightParen,
	/// keyword 'nil'
	Nil,
	/// Keyword boolean 'true'
	True,
	/// Keyword boolean 'false'
	False,
	/// An Identifier
	Ident,
	/// Any Number either f64 or i64
	Number,
	/// Any hexadecimal Number
	HexNumber,
	/// Any String
	String,
	/// An Enum value
	Enum,
}

/// Token
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
	/// Reference to the underlying location
	pub origin: String,
	/// @TODO
	pub offset: usize,
	/// @TODO
	pub line: i16,
	/// Kind of token
	pub kind: TokenKind,
}

impl Token {
	pub fn none() -> Self {
		Self {
			origin: String::default(),
			offset: 0,
			line: 0,
			kind: TokenKind::None,
		}
	}
}
