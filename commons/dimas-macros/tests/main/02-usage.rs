// Copyright © 2024 Stephan Kunz

//! Test correct usage of main macro

#[dimas_macros::main]
async fn main() -> Result<(), std::io::Error> {
    // code inside here
    Ok(())
}
