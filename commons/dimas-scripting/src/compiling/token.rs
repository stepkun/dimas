// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Token for `DiMAS` scripting

use core::fmt::Display;

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
	/// Keyword 'print'
	Print,
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

impl Display for TokenKind {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::None => write!(f, "NONE"),
			Self::Equal => write!(f, "="),
			Self::Colon => write!(f, ":"),
			Self::ColonEqual => write!(f, ":="),
			Self::Plus => write!(f, "+"),
			Self::PlusEqual => write!(f, "+="),
			Self::Minus => write!(f, "-"),
			Self::MinusEqual => write!(f, "-="),
			Self::Star => write!(f, "*"),
			Self::StarEqual => write!(f, "*="),
			Self::Slash => write!(f, "/"),
			Self::SlashEqual => write!(f, "/="),
			Self::Semicolon => write!(f, ";"),
			Self::Ampersand => write!(f, "&"),
			Self::Pipe => write!(f, "|"),
			Self::Caret => write!(f, "^"),
			Self::Tilde => write!(f, "~"),
			Self::And => write!(f, "&&"),
			Self::Or => write!(f, "||"),
			Self::Bang => write!(f, "!"),
			Self::BangEqual => write!(f, "!="),
			Self::EqualEqual => write!(f, "=="),
			Self::Less => write!(f, "<"),
			Self::LessEqual => write!(f, "<="),
			Self::Greater => write!(f, ">"),
			Self::GreaterEqual => write!(f, ">="),
			Self::QMark => write!(f, "?"),
			Self::LeftParen => write!(f, "("),
			Self::RightParen => write!(f, ")"),
			Self::Nil => write!(f, "'nil'"),
			Self::True => write!(f, "'true'"),
			Self::Print => write!(f, "'print'"),
			Self::False => write!(f, "'false'"),
			Self::Ident => write!(f, "an 'Ident'"),
			Self::Number => write!(f, "a 'Number'"),
			Self::HexNumber => write!(f, "a 'HexNumber'"),
			Self::String => write!(f, "a 'String'"),
			Self::Enum => write!(f, "an 'Enum'"),
		}
	}
}

/// Token
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
	/// Reference to the underlying location
	pub origin: String,
	/// @TODO
	pub offset: usize,
	/// @TODO
	pub line: usize,
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
