// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! `BTFactory` errors

#[doc(hidden)]
extern crate alloc;

// region		--- modules
use alloc::{
	string::{FromUtf8Error, String},
	vec::Vec,
};
use evalexpr::{DefaultNumericTypes, EvalexprError};
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("'BTCPP_format' must be '4'")]
	BtCppFormat,

	/// @TODO:
	#[error("children are not allowed for behavior category [{0}]")]
	ChildrenNotAllowed(String),

	/// @TODO:
	#[error("{0}")]
	DimasCoreBehavior(#[from] dimas_core::behavior::error::BehaviorError),

	/// @TODO:
	#[error("decorator [{0}] must have 1 child")]
	DecoratorChildren(String),

	/// @TODO:
	#[error("element [{0}] is not supported")]
	ElementNotSupported(String),

	/// @TODO:
	#[error("attribute 'main_tree_to_execute' not allowed in subtree definition")]
	MainTreeNotAllowed,

	/// @TODO:
	#[error("missig attribute 'ID' in tag [{0}]")]
	MissingId(String),

	/// @TODO:
	#[error("no main tree provided")]
	NoMainTree,

	/// @TODO:
	#[error("no 'main_tree_to_execute' provided")]
	NoTreeToExecute,

	/// @TODO:
	#[error("Error parsing expression in port value: {0}")]
	PortExpressionInvalid(#[from] EvalexprError<DefaultNumericTypes>),

	/// @TODO:
	#[error("invalid type [{0}] for variable [{1}]")]
	PortExpressionInvalidType(String, String),

	/// @TODO:
	#[error("variable in blackboard pointer [{0}] has no type")]
	PortExpressionMissingType(String),

	/// @TODO:
	#[error("port name [{0}] does not match nodes [{1}] port list: {2:?}")]
	PortInvalid(String, String, Vec<String>),

	/// @TODO:
	#[error("root element must be named 'root'")]
	RootName,

	/// @TODO:
	#[error("{0}")]
	RoXmlTree(#[from] roxmltree::Error),

	/// @TODO:
	#[error("unkown behavior [{0}]")]
	UnknownBehavior(String),

	/// @TODO:
	#[error("unkown element [{0}]")]
	UnknownElement(String),

	/// @TODO:
	#[error("processing instructions are not supported")]
	UnkownProcessingInstruction,

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(String, String, u32),

	/// @TODO:
	#[error("Error parsing UTF8: {0}")]
	Utf8(#[from] FromUtf8Error),
}
// region:		--- Error
