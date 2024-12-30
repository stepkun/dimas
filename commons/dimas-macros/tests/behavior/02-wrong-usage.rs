// Copyright © 2024 Stephan Kunz

//! Test correct usage of behavior macro on struct block

extern crate alloc;

#[dimas_macros::behavior(Action)]
struct S1 {}

#[dimas_macros::behavior(Action)]
impl S1 {}


#[dimas_macros::behavior(SyncAction)]
struct S2 {}

#[dimas_macros::behavior(SyncAction)]
impl S2 {}


#[dimas_macros::behavior(SyncAction)]
struct S3 {
    f1: i32,
}

#[dimas_macros::behavior(SyncAction)]
impl S3 {
    async fn tick(&mut self) -> BehaviorResult {
        Ok(BehaviorStatus::Success)
    }
}


fn main() {}