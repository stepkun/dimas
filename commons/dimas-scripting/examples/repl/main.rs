//! `DiMAS` scripting `REPL` example
//! Copyright © 2025 Stephan Kunz

use std::io::{Write, stdin, stdout};

use dimas_scripting::{DefaultEnvironment, Runtime};

fn repl() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let mut input = String::new();

	print!("> ");
	let _ = stdout().flush();
	loop {
		match stdin().read_line(&mut input) {
			Ok(len) => {
				if len > 0 {
					// ignore CR/LF only input
					if input.len() > 1 {
						// print!("{}", &input);
						runtime.parse(&input).map_or_else(
							|err| {
								println!("parsing error: {err}");
							},
							|chunk| {
								//chunk.disassemble("created chunk");
								runtime.clear();
								if let Err(error) = runtime.execute(&chunk, &mut env) {
									println!("execution error: {error}");
								} else {
									for c in runtime.stdout() {
										print!("{}", *c as char);
									}
								}
							},
						);
					}
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

fn main() {
	// initialize tracing/logging
	//init_tracing();
	repl();
}
