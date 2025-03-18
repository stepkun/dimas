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
					// ignore CR/LF only input
					if input.len() > 1 {
						print!("{}", &input);
						let mut parser = Parser::new(&input);
						parser.parse().map_or_else(
							|err| {
								println!("parsing error: {err}");
							},
							|chunk| {
								if let Err(error) = vm.run(&chunk) {
									println!("execution error: {error}");
								};
							},
						);
					};
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
