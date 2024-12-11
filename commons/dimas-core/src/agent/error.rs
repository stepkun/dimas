// Copyright © 2024 Stephan Kunz

//! [`Agent`] errors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `dimas-core` error type.
#[derive(Error, Debug)]
pub enum Error {}
// region:		--- Error
