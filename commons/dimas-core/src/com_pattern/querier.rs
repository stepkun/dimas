// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Querier contract

use anyhow::Result;

pub trait Querier {
	fn query() -> Result<()>;
}
