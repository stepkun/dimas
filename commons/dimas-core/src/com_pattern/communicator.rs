// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]
#![allow(unused)]

//! `communicator`defines the interfaces

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::future::Future;

use crate::{message_types::Message, Activity, Agent};

use super::subscriber::Subscriber;
// endregion:   --- modules

// region:      --- CommunicatorFactory
/// `CommunicatorFactory` trait.
#[allow(clippy::module_name_repetitions)]
pub trait CommunicatorFactory {
	/// Factory method for subscriber creation
	/// # Errors
	/// - if a communicator does not implement that feature
	fn create_subscriber<CB, F>(&mut self, selector: &str, callback: CB) -> Result<()>
	where
		CB: FnMut(Agent, Message) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static;
}
// endregion:   --- CommunicatorFactory

