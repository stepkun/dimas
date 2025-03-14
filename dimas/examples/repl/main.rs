//! `DiMAS` scripting `REPL` example
//! Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

use std::io::{Write, stdin, stdout};

use dimas::prelude::*;
use dimas_core::scripting::{Parser, VM};

fn repl() {
	let mut vm = VM::default();
	let mut input = String::new();

	print!("> ");
	let _ = stdout().flush();
	loop {
		match stdin().read_line(&mut input) {
			Ok(len) => {
				if len > 0 {
					print!("{}", &input);
					let parser = Parser::new(&input);
					parser.parse().map_or_else(
						|_| {
							println!("parsing error");
						},
						|chunk| {
							if let Err(error) = vm.run(&chunk) {
								println!("execution error: {error}");
							};
						},
					);
					input.clear();
					print!("> ");
					let _ = stdout().flush();
				} else {
					println!("bye");
					break;
				}
			}
			Err(_) => todo!(),
		}
	}
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	//init_tracing();
	repl();
	Ok(())
}
