// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Observer contract

use anyhow::Result;

pub trait Observer {
	fn observe() -> Result<()>;
}
