// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Queryable contract

use anyhow::Result;

pub trait Queryable {
	fn request() -> Result<()>;
	fn response() -> Result<()>;
}
