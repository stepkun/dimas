// Copyright © 2024 Stephan Kunz

//! Helper functions and structs
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
#[doc(hidden)]
extern crate std;

// region:		--- modules
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
	//@TODO: set tracing values to ("dimas=warn,zenoh=warn")
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
