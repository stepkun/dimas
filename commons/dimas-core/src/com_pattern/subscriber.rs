// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Subscriber contract

use anyhow::Result;

pub trait Subscriber {
    fn subscribe() -> Result<()>;
}
