// Copyright © 2024 Stephan Kunz

//! Helper functions and structs
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use tracing_subscriber::EnvFilter;
// endregion:	--- modules

// region:    --- tracing
/// Initialize tracing
pub fn init_tracing() {
	#[cfg(feature = "std")]
	let subscriber = std::env::var("RUST_LOG").map_or_else(
		|_| tracing_subscriber::fmt().with_env_filter("dimas=warn,zenoh=warn"),
		|content| {
			let levels = content.split(',');
			if levels.count() == 1 {
				tracing_subscriber::fmt()
					.with_env_filter(EnvFilter::new(std::format!("dimas={content},zenoh=warn")))
			} else {
				tracing_subscriber::fmt().with_env_filter(EnvFilter::new(content))
			}
		},
	);
	#[cfg(not(feature = "std"))]
	let subscriber = tracing_subscriber::fmt();

	let subscriber = subscriber
		.compact()
		.with_file(true)
		.with_line_number(true)
		.with_thread_ids(true)
		.with_thread_names(true)
		.with_level(true)
		.with_target(true)
		.finish();

	let _ = tracing::subscriber::set_global_default(subscriber);
}
// endregion: --- tracing

// region:    --- helper
/// create selector
#[must_use]
pub fn selector_from(topic: &str, mut prefix: Option<&String>) -> String {
	prefix.take().map_or(topic.to_string(), |prefix| {
		let mut result = String::from(prefix);
		result.push('/');
		result.push_str(topic);
		result
	})
}

/// create request selector
#[must_use]
pub fn request_selector_from(selector: &str) -> String {
	let mut result = String::from(selector);
	result.push_str("?request");
	result
}

/// create cancel selector
#[must_use]
pub fn cancel_selector_from(selector: &str) -> String {
	let mut result = String::from(selector);
	result.push_str("?cancel");
	result
}

/// create feedback selector
#[must_use]
pub fn feedback_selector_from(selector: &str, id: &str) -> String {
	let mut result = String::from(selector);
	result.push_str("/feedback/");
	result.push_str(id);
	result
}
// endregion: --- helper
