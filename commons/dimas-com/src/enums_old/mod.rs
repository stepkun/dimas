// Copyright © 2024 Stephan Kunz

use std::sync::Arc;
use zenoh::Session;

use crate::traits_old::CommunicatorImplementationMethods;

/// the known implementations of communicators
#[derive(Debug)]
pub enum CommunicatorImplementation {
	/// zenoh
	Zenoh(crate::zenoh_old::CommunicatorOld),
}

impl CommunicatorImplementationMethods for CommunicatorImplementation {}

impl CommunicatorImplementation {
	/// extract session
	#[must_use]
	#[allow(clippy::match_wildcard_for_single_variants)]
	pub fn session(&self) -> Arc<Session> {
		match self {
			Self::Zenoh(communicator) => communicator.session(),
		}
	}
}
