// Copyright © 2025 Stephan Kunz

//! Tests of lexing functionality

use dimas_scripting::{Lexer, TokenKind};

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
enum Color {
	RED = 1,
	BLUE = 2,
	GREEN = 3,
}

#[allow(clippy::cognitive_complexity)]
fn lexing_tokens(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::ColonEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Equal
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Plus
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Minus
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Star
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Slash
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::PlusEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::MinusEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::StarEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::SlashEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Semicolon
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Bang
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Ampersand
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Pipe
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Caret
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Tilde
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::And
	);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Or);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::EqualEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::BangEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Less
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::LessEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Greater
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::GreaterEqual
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Colon
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::QMark
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::LeftParen
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::RightParen
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing() {
	let tokens = ":= = + - * / += -= *= /= ; ! & | ^ ~ && || == != < <= > >= : ? ( )";
	lexing_tokens(tokens);
	let tokens2 = ":==+-*/+=-=*=/=;!&|^~&&||==!=<<=>>=:?()";
	lexing_tokens(tokens2);
}

#[test]
fn lexing_keywords() {
	let tokens = "true false";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::True
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::False
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_idents() {
	let tokens = "a_name _another_name _aThirdName_";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Ident
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Ident
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Ident
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_numbers() {
	let tokens = "123 123.0 123.456 0.123 0x123";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::IntNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::HexNumber
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_hex() {
	let tokens = "0x123 0xABC 0xabc 0xa1b2c3";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::HexNumber
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_strings() {
	let tokens = "'teststring' 'another_string'";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::String
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::String
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
#[ignore = "not yet implemented"]
fn lexing_enums() {
	let tokens = "RED BLUE GREEN";
	// @TODO
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Enum
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Enum
	);
	assert_eq!(
		lexer.next().expect("snh").expect("snh").kind,
		TokenKind::Enum
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}
