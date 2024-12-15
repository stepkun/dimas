// Copyright © 2024 Stephan Kunz

//! Commands for `DiMAS` control & monitoring programs

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::messages::AboutEntity;
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use anyhow::Result;
use dimas_com::{traits_old::CommunicatorImplementationMethods, zenoh_old::CommunicatorOld};
use dimas_core::{enums::Signal, message_types::Message, utils::selector_from, OperationState};
#[cfg(feature = "std")]
use std::collections::HashMap;
use tracing::{event, instrument, Level};
// endregion:	--- modules

// region:		--- set_state
/// Set the [`OperationState`] of `DiMAS` entities
/// # Errors
#[cfg(feature = "std")]
#[instrument(level = Level::DEBUG, skip_all)]
pub fn set_state(
	com: &CommunicatorOld,
	base_selector: &String,
	state: Option<OperationState>,
) -> Result<Vec<AboutEntity>> {
	event!(Level::DEBUG, "set_state");
	let mut map: HashMap<String, AboutEntity> = HashMap::new();

	let selector = selector_from("signal", Some(base_selector));
	let message = Message::encode(&Signal::State { state });
	// set state for entities matching the selector
	com.get(
		&selector,
		Some(message),
		Some(&mut |response| -> Result<()> {
			let response: AboutEntity = response.decode()?;
			map.entry(response.zid().to_string())
				.or_insert(response);
			Ok(())
		}),
	)?;

	let result: Vec<AboutEntity> = map.values().cloned().collect();

	Ok(result)
}
// endregion:	--- set_state

// region:		--- shutdown
/// Shutdown of `DiMAS` entities
/// # Errors
#[cfg(feature = "std")]
#[instrument(level = Level::DEBUG, skip_all)]
pub fn shutdown(com: &CommunicatorOld, base_selector: &String) -> Result<Vec<AboutEntity>> {
	event!(Level::DEBUG, "shutdown");
	let mut map: HashMap<String, AboutEntity> = HashMap::new();

	let selector = selector_from("signal", Some(base_selector));
	let message = Message::encode(&Signal::Shutdown);
	// set state for entities matching the selector
	com.get(
		&selector,
		Some(message),
		Some(&mut |response| -> Result<()> {
			let response: AboutEntity = response.decode()?;
			map.entry(response.zid().to_string())
				.or_insert(response);
			Ok(())
		}),
	)?;

	let result: Vec<AboutEntity> = map.values().cloned().collect();

	Ok(result)
}
// endregion:	--- shutdown
