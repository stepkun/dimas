// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]

//! Tests

use dimas_core::scripting::{Lexer, TokenKind};
enum Color {
	RED = 1,
	BLUE = 2,
	GREEN = 3,
}

#[allow(clippy::cognitive_complexity)]
fn lexing_tokens(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::ColonEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Equal);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Plus);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Minus);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Star);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Slash);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::PlusEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::MinusEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::StarEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::SlashEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Semicolon);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Not);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ampersand);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Pipe);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Circonflex);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::And);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Or);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::EqualEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::NotEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Less);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::LessEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Greater);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::GreaterEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Colon);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::QMark);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::LeftParen);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::RightParen);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

fn lexing_keywords(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::True);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::False);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

fn lexing_idents(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

fn lexing_numbers(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::Number(123.0f64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::Number(123.0f64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::Number(123.456f64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::Number(0.123f64)
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

fn lexing_hex(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::HexNumber(291i64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::HexNumber(2748i64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::HexNumber(2748i64)
	);
	assert_eq!(
		lexer.next().unwrap().unwrap().kind,
		TokenKind::HexNumber(10_597_059i64)
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

fn lexing_strings(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::String);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::String);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing() {
	let tokens = ":= = + - * / += -= *= /= ; ! & | ^ && || == != < <= > >= : ? ( )";
	lexing_tokens(tokens);
	let tokens2 = ":==+-*/+=-=*=/=;!&|^&&||==!=<<=>>=:?()";
	lexing_tokens(tokens2);
	let keywords = "true false";
	lexing_keywords(keywords);
	let idents = "a_name _another_name _aThirdName_";
	lexing_idents(idents);
	let numbers = "123 123.0 123.456 0.123";
	lexing_numbers(numbers);
	let hex_numbers = "0x123 0xABC 0xabc 0xa1b2c3";
	lexing_hex(hex_numbers);
	let strings = "'teststring' 'another_string'";
	lexing_strings(strings);
	let enums = "RED BLUE GREEN";
	// @TODO
}

#[test]
const fn parsing() {}

#[test]
const fn executing() {}
