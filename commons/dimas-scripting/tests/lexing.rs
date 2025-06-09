// Copyright © 2025 Stephan Kunz

//! Tests of lexing functionality

use std::collections::BTreeMap;

use dimas_scripting::compiling::{Lexer, TokenKind};

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
enum Color {
	RED = 1,
	BLUE = 2,
	GREEN = 3,
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
fn lexing_tokens(tokens: &str) {
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::ColonEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Equal);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Plus);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Minus);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Star);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Slash);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::PlusEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::MinusEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::StarEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::SlashEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Semicolon);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Bang);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Ampersand);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Pipe);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Caret);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Tilde);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::And);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Or);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::EqualEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::BangEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Less);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::LessEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Greater);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::GreaterEqual);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Colon);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::QMark);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::LeftParen);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::RightParen);
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
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::True);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::False);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_idents() {
	let tokens = "a_name _another_name _aThirdName_";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Ident);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Ident);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Ident);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_numbers() {
	let tokens = "123 123.0 123.456 0.123 0x123";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::IntNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::FloatNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::FloatNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::FloatNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::HexNumber);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_hex() {
	let tokens = "0x123 0xABC 0xabc 0xa1b2c3";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::HexNumber);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_strings() {
	let tokens = "'teststring' 'another_string'";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::String);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::String);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_enums() {
	let tokens = "First SECOND Third";
	// @TODO
	let mut enums: BTreeMap<String, i8> = BTreeMap::default();
	enums.insert("First".into(), 1);
	enums.insert("SECOND".into(), 2);
	enums.insert("Third".into(), 3);

	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Enum);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Enum);
	assert_eq!(lexer.next().expect("snh").expect("snh").kind, TokenKind::Enum);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}
