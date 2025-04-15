// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! [`BehaviorTreeFactory`] and [`XmlParser`] errors
//!

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::{
	string::{FromUtf8Error, String},
	vec::Vec,
};
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas` error type
#[derive(Error, Debug)]
pub enum Error {
	/// Behavior is not registered
	#[error("behavior [{0}] is not registered")]
	BehaviorNotRegistered(String),
	/// A wron BTCPP version is given
	#[error("'BTCPP_format' must be '4'")]
	BtCppFormat,
	/// Unsupported XML element:
	#[error("the Decorator [{0}] has more than 1 child")]
	DecoratorOnlyOneChild(String),
	/// Unsupported XML element:
	#[error("element [{0}] is not supported")]
	ElementNotSupported(String),
	/// Attribut 'ID' is missing
	#[error("missing attribute 'ID' in tag [{0}]")]
	MissingId(String),
	/// The main tree information is missing
	#[error("no 'main_tree_to_execute' provided")]
	NoTreeToExecute,
	/// Processing instruction
	#[error("processing instruction [{0}] is not supported")]
	UnsupportedProcessingInstruction(String),
	/// Wrong name for the root element
	#[error("root element must be named 'root'")]
	WrongRootName,
	/// Passthrough for roxmltree Errors
	#[error("{0}")]
	XmlParser(#[from] roxmltree::Error),

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),
}
// region:		--- Error
