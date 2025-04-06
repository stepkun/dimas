//! `DiMAS` scripting `REPL` example
//! Copyright © 2025 Stephan Kunz

use std::io::{Write, stdin, stdout};

use dimas_scripting::{DefaultEnvironment, Parser, VM};

fn repl() {
	let env = DefaultEnvironment::default();
	let mut vm = VM::new(&env);
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
						let mut parser = Parser::new(&input);
						parser.parse().map_or_else(
							|err| {
								println!("parsing error: {err}");
							},
							|mut chunk| {
								//chunk.disassemble("created chunk");
								let mut stdout: Vec<u8> = Vec::new();
								if let Err(error) = vm.run(&mut chunk, &mut stdout) {
									println!("execution error: {error}");
								} else {
									for c in stdout {
										print!("{}", c as char);
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
