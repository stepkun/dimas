// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_config::Config;

#[test]
fn config() {
	Config::default();
	// @TODO: reactivate
	//assert!(Config::local().is_ok());
	//assert!(Config::client().is_ok());
	//assert!(Config::peer().is_ok());
	//assert!(Config::router().is_ok());
	//assert!(Config::from_file("default.json5").is_ok());
	//assert!(Config::from_file("non_existent.json5").is_err());
}
