// Copyright © 2024 Stephan Kunz

//! Macro tests

#[dimas_macros::main(additional_threads = 5)]
async fn main() -> core::result::Result<(), std::io::Error> {
	println!("Hello world");
	Ok(())
}

#[test]
fn main_macro() -> core::result::Result<(), std::io::Error> {
	main()
}
