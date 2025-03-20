// Copyright © 2025 Stephan Kunz

//! Bsic tests for bytecode interpreter

use dimas_core::scripting::{
	VM,
	execution::{
		Chunk,
		opcodes::{OP_ADD, OP_CONSTANT, OP_DIVIDE, OP_NEGATE, OP_RETURN},
	},
};

#[test]
fn first_test() {
	let mut chunk = Chunk::default();

	let constant = chunk.add_constant(1.2);
	chunk.write(OP_CONSTANT, 123);
	chunk.write(constant, 123);
	let constant = chunk.add_constant(3.4);
	chunk.write(OP_CONSTANT, 123);
	chunk.write(constant, 123);
	chunk.write(OP_ADD, 123);

	let constant = chunk.add_constant(5.6);
	chunk.write(OP_CONSTANT, 124);
	chunk.write(constant, 124);
	chunk.write(OP_DIVIDE, 124);

	chunk.write(OP_NEGATE, 125);
	chunk.write(OP_RETURN, 125);

	chunk.disassemble("test chunk");

	let mut vm = VM::default();
	assert!(vm.run(&chunk).is_ok());
}
