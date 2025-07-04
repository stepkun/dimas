// Copyright © 2024 Stephan Kunz

//! List commands for `DiMAS` control & monitoring programs

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::messages::{AboutEntity, PingEntity, ScoutingEntity};
use alloc::vec::Vec;
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
};
use chrono::Local;
use core::time::Duration;
use dimas_com::{traits::CommunicatorImplementationMethods, zenoh::Communicator};
use dimas_config::Config;
use dimas_core::{Result, enums::Signal, message_types::Message, utils::selector_from};
#[cfg(feature = "std")]
use std::collections::HashMap;
use zenoh::{
	Wait,
	config::{Locator, WhatAmI},
};
// endregion:	--- modules

// region:		--- about_list
/// Fetch a list of about messages from all reachable `DiMAS` entities
/// # Errors
#[cfg(feature = "std")]
pub fn about_list(com: &Communicator, base_selector: &String) -> Result<Vec<AboutEntity>> {
	let mut map: HashMap<String, AboutEntity> = HashMap::new();

	let selector = selector_from("signal", Some(base_selector));
	let message = Message::encode(&Signal::About);
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
// endregion:	--- about_list

// region:		--- ping_list
/// Ping all reachable `DiMAS` entities
/// # Errors
#[cfg(feature = "std")]
pub fn ping_list(com: &Communicator, base_selector: &String) -> Result<Vec<(PingEntity, i64)>> {
	let mut map: HashMap<String, (PingEntity, i64)> = HashMap::new();

	let selector = selector_from("signal", Some(base_selector));
	let sent = Local::now()
		.naive_utc()
		.and_utc()
		.timestamp_nanos_opt()
		.unwrap_or(0);
	let message = Message::encode(&Signal::Ping { sent });
	// set state for entities matching the selector
	com.get(
		&selector,
		Some(message),
		Some(&mut |response| -> Result<()> {
			let received = Local::now()
				.naive_utc()
				.and_utc()
				.timestamp_nanos_opt()
				.unwrap_or(0);

			let response: PingEntity = response.decode()?;
			let roundtrip = received - sent;
			map.entry(response.zid().to_string())
				.or_insert((response, roundtrip));
			Ok(())
		}),
	)?;

	let result: Vec<(PingEntity, i64)> = map.values().cloned().collect();

	Ok(result)
}
// endregion:	--- ping_list

// region:		--- scouting_list
/// Scout for `DiMAS` entities, sorted by zid of entity
/// # Errors
#[cfg(feature = "std")]
pub fn scouting_list(config: &Config) -> Result<Vec<ScoutingEntity>> {
	let mut map: HashMap<String, ScoutingEntity> = HashMap::new();
	let what = WhatAmI::Router | WhatAmI::Peer | WhatAmI::Client;
	let receiver = zenoh::scout(what, config.zenoh_config().to_owned()).wait()?;

	while let Ok(Some(hello)) = receiver.recv_timeout(Duration::from_millis(250)) {
		let zid = hello.zid().to_string();
		let locators: Vec<String> = hello
			.locators()
			.iter()
			.map(Locator::to_string)
			.collect();

		let entry = ScoutingEntity::new(zid.clone(), hello.whatami().to_string(), locators);
		map.entry(zid).or_insert(entry);
	}
	let result: Vec<ScoutingEntity> = map.values().cloned().collect();

	Ok(result)
}
// endregion:	--- scouting_list
