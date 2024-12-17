// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Publisher contract

use anyhow::Result;

pub trait Publisher {
	fn publish() -> Result<()>;
}
