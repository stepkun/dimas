// Copyright © 2024 Stephan Kunz
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::match_same_arms)]

//! Commandline tool for `DiMAS`

// region:		--- modules
mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
// endregion:	--- modules

// region:		--- Cli
#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct DimasctlArgs {
	/// Optional selector for the instances to operate on
	selector: Option<String>,

	#[clap(subcommand)]
	command: DimasctlCommand,
}
// endregion:	--- Cli

// region:		--- Commands
#[derive(Debug, Subcommand)]
enum DimasctlCommand {
	/// List running `DiMAS` entities
	List,
	/// Ping entities
	Ping,
	/// Scout for `Zenoh` entities
	Scout,
	/// Set state of entities
	SetState,
	/// Shurdown entities
	Shutdown,
}
// endregion:	--- Commands#[allow(clippy::unnecessary_wraps)]

fn main() -> Result<()> {
	let args = DimasctlArgs::parse();


	match &args.command {
		DimasctlCommand::List => {
		}
		DimasctlCommand::Ping => {
		}
		DimasctlCommand::Scout => {
		}
		DimasctlCommand::SetState => {
		}
		DimasctlCommand::Shutdown => {
		}
	}
	Ok(())
}
