// Copyright © 2024 Stephan Kunz

//! Test correct usage of behavior macro on struct block

extern crate alloc;

use dimas_behavior::behavior::{BehaviorStatus, BehaviorResult};
use dimas_macros::behavior;

#[behavior(Action)]
struct S11 {
    #[bhvr(default)]    // @TODO: remove this default
    f1: i32,
    #[bhvr(default)]
    f2: i32,
    #[bhvr(default = "1")]
    f3: i32,
    #[bhvr(default = "String::new()")]
    f4: String,
}

#[behavior(Action)]
impl S11 {
    async fn on_start(&mut self) -> BehaviorResult {
        Ok(BehaviorStatus::Running)
    }

    async fn on_running(&mut self) -> BehaviorResult {
        Ok(BehaviorStatus::Success)
    }
}


#[behavior(SyncAction)]
struct S21 {}

#[behavior(SyncAction)]
impl S21 {
    async fn tick(&mut self) -> BehaviorResult {
        Ok(BehaviorStatus::Success)
    }
}


fn main() {}