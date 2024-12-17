// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Observable contract

use anyhow::Result;

pub trait Observable {
	fn request() -> Result<()>;
	fn feedback() -> Result<()>;
	fn response() -> Result<()>;
}
