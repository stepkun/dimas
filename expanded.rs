#![feature(prelude_import)]
#![no_std]
#![allow(unused)]
//! Library for configuration
//!
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
#[doc(hidden)]
extern crate alloc;
#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;
pub mod builtin {
    //! Buit in behaviors of `DiMAS`
    #[doc(hidden)]
    extern crate alloc;
    pub mod control {
        //! Built in control nodes of `DiMAS`
        mod if_then_else {
            #![allow(clippy::module_name_repetitions)]
            //! Built in if-then-else node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            use tracing::warn;
            /// IfThenElseNode must have exactly 2 or 3 children. This node is NOT reactive.
            ///
            /// The first child is the "statement" of the if.
            ///
            /// If that return SUCCESS, then the second child is executed.
            ///
            /// Instead, if it returned FAILURE, the third child is executed.
            ///
            /// If you have only 2 children, this node will return FAILURE whenever the
            /// statement returns FAILURE.
            ///
            /// This is equivalent to add AlwaysFailure as 3rd child.
            pub struct IfThenElse {
                child_idx: usize,
            }
            impl IfThenElse {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self { child_idx: 0 };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("IfThenElse"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl IfThenElse {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            let children_count = bhvr_.children.len();
                            if !(2..=3).contains(&children_count) {
                                return Err(
                                    BehaviorError::NodeStructure(
                                        "IfThenElseNode must have either 2 or 3 children."
                                            .to_string(),
                                    ),
                                );
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            if self_.child_idx == 0 {
                                let status = bhvr_.children[0].execute_tick().await?;
                                match status {
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Success => self_.child_idx += 1,
                                    BehaviorStatus::Failure => {
                                        if children_count == 3 {
                                            self_.child_idx = 2;
                                        } else {
                                            return Ok(BehaviorStatus::Failure);
                                        }
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "Node name here".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                    _ => {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event commons/dimas-config/src/builtin/control/if_then_else.rs:63",
                                                    "dimas_config::builtin::control::if_then_else",
                                                    ::tracing::Level::WARN,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "commons/dimas-config/src/builtin/control/if_then_else.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(63u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "dimas_config::builtin::control::if_then_else",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::WARN
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::WARN
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!(
                                                                        "Condition node of IfThenElseNode returned Skipped",
                                                                    ) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    }
                                }
                            }
                            if self_.child_idx > 0 {
                                let status = bhvr_
                                    .children[self_.child_idx]
                                    .execute_tick()
                                    .await?;
                                match status {
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    status => {
                                        bhvr_.reset_children().await;
                                        self_.child_idx = 0;
                                        return Ok(status);
                                    }
                                }
                            }
                            Err(
                                BehaviorError::NodeStructure(
                                    "Something unexpected happened in IfThenElseNode"
                                        .to_string(),
                                ),
                            )
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.child_idx = 0;
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use if_then_else::*;
        mod fallback {
            #![allow(clippy::module_name_repetitions)]
            //! Built in fallback node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The FallbackNode is used to try different strategies,
            /// until one succeeds.
            /// If any child returns RUNNING, previous children will NOT be ticked again.
            ///
            /// - If all the children return FAILURE, this node returns FAILURE.
            ///
            /// - If a child returns RUNNING, this node returns RUNNING.
            ///
            /// - If a child returns SUCCESS, stop the loop and return SUCCESS.
            pub struct Fallback {
                child_idx: usize,
                all_skipped: bool,
            }
            impl Fallback {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        child_idx: 0,
                        all_skipped: true,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Fallback"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Fallback {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            if bhvr_.status == BehaviorStatus::Idle {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            while self_.child_idx < bhvr_.children.len() {
                                let cur_child = &mut bhvr_.children[self_.child_idx];
                                let _prev_status = cur_child.status();
                                let child_status = cur_child.execute_tick().await?;
                                self_.all_skipped
                                    &= child_status == BehaviorStatus::Skipped;
                                match &child_status {
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure | BehaviorStatus::Skipped => {
                                        self_.child_idx += 1;
                                    }
                                    BehaviorStatus::Success => {
                                        bhvr_.reset_children().await;
                                        self_.child_idx = 0;
                                        return Ok(BehaviorStatus::Success);
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "Name here".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                };
                            }
                            if self_.child_idx == bhvr_.children.len() {
                                bhvr_.reset_children().await;
                                self_.child_idx = 0;
                            }
                            if self_.all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Failure)
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.child_idx = 0;
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use fallback::*;
        mod reactive_fallback {
            #![allow(clippy::module_name_repetitions)]
            //! Built in reactive-fallback node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The ReactiveFallback is similar to a ParallelNode.
            /// All the children are ticked from first to last:
            ///
            /// - If a child returns RUNNING, continue to the next sibling.
            /// - If a child returns FAILURE, continue to the next sibling.
            /// - If a child returns SUCCESS, stop and return SUCCESS.
            ///
            /// If all the children fail, than this node returns FAILURE.
            ///
            /// IMPORTANT: to work properly, this node should not have more than
            ///            a single asynchronous child.
            pub struct ReactiveFallback {}
            impl ReactiveFallback {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("ReactiveFallback"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl ReactiveFallback {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            let mut all_skipped = true;
                            bhvr_.status = BehaviorStatus::Running;
                            for index in 0..bhvr_.children.len() {
                                let cur_child = &mut bhvr_.children[index];
                                let child_status = cur_child.execute_tick().await?;
                                all_skipped &= child_status == BehaviorStatus::Skipped;
                                match &child_status {
                                    BehaviorStatus::Running => {
                                        for i in 0..index {
                                            bhvr_.halt_child_idx(i).await?;
                                        }
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure => {}
                                    BehaviorStatus::Success => {
                                        bhvr_.reset_children().await;
                                        return Ok(BehaviorStatus::Success);
                                    }
                                    BehaviorStatus::Skipped => {
                                        bhvr_.halt_child_idx(index).await?;
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "Name here".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                };
                            }
                            bhvr_.reset_children().await;
                            if all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Failure)
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use reactive_fallback::*;
        mod parallel {
            #![allow(clippy::module_name_repetitions)]
            //! Built in parallel node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            use hashbrown::HashSet;
            /// The ParallelNode execute all its children
            /// __concurrently__, but not in separate threads!
            ///
            /// Even if this may look similar to ReactiveSequence,
            /// this Control Node is the __only__ one that can have
            /// multiple children RUNNING at the same time.
            ///
            /// The Node is completed either when the THRESHOLD_SUCCESS
            /// or THRESHOLD_FAILURE number is reached (both configured using ports).
            ///
            /// If any of the thresholds is reached, and other children are still running,
            /// they will be halted.
            ///
            /// Note that threshold indexes work as in Python:
            /// https://www.i2tutorials.com/what-are-negative-indexes-and-why-are-they-used/
            ///
            /// Therefore -1 is equivalent to the number of children.
            pub struct Parallel {
                success_threshold: i32,
                failure_threshold: i32,
                completed_list: HashSet<usize>,
                success_count: usize,
                failure_count: usize,
            }
            impl Parallel {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        success_threshold: -1,
                        failure_threshold: -1,
                        completed_list: <HashSet<usize>>::default(),
                        success_count: 0,
                        failure_count: 0,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Parallel"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Parallel {
                #[allow(clippy::cast_sign_loss)]
                fn success_threshold(&self, n_children: i32) -> usize {
                    if self.success_threshold < 0 {
                        (n_children + self.success_threshold + 1).max(0) as usize
                    } else {
                        self.success_threshold as usize
                    }
                }
                #[allow(clippy::cast_sign_loss)]
                fn failure_threshold(&self, n_children: i32) -> usize {
                    if self.failure_threshold < 0 {
                        (n_children + self.failure_threshold + 1).max(0) as usize
                    } else {
                        self.failure_threshold as usize
                    }
                }
                fn clear(&mut self) {
                    self.completed_list.clear();
                    self.success_count = 0;
                    self.failure_count = 0;
                }
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_possible_wrap)]
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.success_threshold = bhvr_
                                .config
                                .get_input("success_count")
                                .unwrap_or_else(|_| ::core::panicking::panic(
                                    "not yet implemented",
                                ));
                            self_.failure_threshold = bhvr_
                                .config
                                .get_input("failure_count")
                                .unwrap_or_else(|_| ::core::panicking::panic(
                                    "not yet implemented",
                                ));
                            let children_count = bhvr_.children.len();
                            if children_count
                                < self_.success_threshold(bhvr_.children.len() as i32)
                            {
                                return Err(
                                    BehaviorError::NodeStructure(
                                        #[allow(clippy::match_same_arms)]
                                        "Number of children is less than the threshold. Can never succeed."
                                            .to_string(),
                                    ),
                                );
                            }
                            if children_count
                                < self_.failure_threshold(bhvr_.children.len() as i32)
                            {
                                return Err(
                                    BehaviorError::NodeStructure(
                                        "Number of children is less than the threshold. Can never fail."
                                            .to_string(),
                                    ),
                                );
                            }
                            let mut skipped_count = 0;
                            for i in 0..children_count {
                                if !self_.completed_list.contains(&i) {
                                    let child = &mut bhvr_.children[i];
                                    match child.execute_tick().await? {
                                        BehaviorStatus::Skipped => skipped_count += 1,
                                        BehaviorStatus::Success => {
                                            self_.completed_list.insert(i);
                                            self_.success_count += 1;
                                        }
                                        BehaviorStatus::Failure => {
                                            self_.completed_list.insert(i);
                                            self_.failure_count += 1;
                                        }
                                        BehaviorStatus::Running => {}
                                        BehaviorStatus::Idle => {
                                            ::core::panicking::panic("not yet implemented")
                                        }
                                    }
                                }
                                let required_success_count = self_
                                    .success_threshold(bhvr_.children.len() as i32);
                                if self_.success_count >= required_success_count
                                    || (self_.success_threshold < 0
                                        && (self_.success_count + skipped_count)
                                            >= required_success_count)
                                {
                                    self_.clear();
                                    bhvr_.reset_children().await;
                                    return Ok(BehaviorStatus::Success);
                                }
                                if (children_count - self_.failure_count)
                                    < required_success_count
                                    || self_.failure_count
                                        == self_.failure_threshold(bhvr_.children.len() as i32)
                                {
                                    self_.clear();
                                    bhvr_.reset_children().await;
                                    return Ok(BehaviorStatus::Failure);
                                }
                            }
                            if skipped_count == children_count {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Running)
                            }
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let mut port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            port_info.set_default(-1);
                            ("success_count", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        let (name, port_info) = {
                            let mut port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            port_info.set_default(1);
                            ("failure_count", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_children().await;
                        }
                    })
                }
            }
        }
        pub use parallel::*;
        mod parallel_all {
            #![allow(clippy::module_name_repetitions)]
            //! Built in parallel-all node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            use hashbrown::HashSet;
            /// The ParallelAllNode execute all its children
            /// __concurrently__, but not in separate threads!
            ///
            /// It differs in the way ParallelNode works because the latter may stop
            /// and halt other children if a certain number of SUCCESS/FAILURES is reached,
            /// whilst this one will always complete the execution of ALL its children.
            ///
            /// Note that threshold indexes work as in Python:
            /// https://www.i2tutorials.com/what-are-negative-indexes-and-why-are-they-used/
            ///
            /// Therefore -1 is equivalent to the number of children.
            pub struct ParallelAll {
                failure_threshold: i32,
                completed_list: HashSet<usize>,
                failure_count: usize,
            }
            impl ParallelAll {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        failure_threshold: -1,
                        completed_list: <HashSet<usize>>::default(),
                        failure_count: 0,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("ParallelAll"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl ParallelAll {
                #[allow(clippy::cast_sign_loss)]
                fn failure_threshold(&self, n_children: i32) -> usize {
                    if self.failure_threshold < 0 {
                        (n_children + self.failure_threshold + 1).max(0) as usize
                    } else {
                        self.failure_threshold as usize
                    }
                }
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_possible_wrap)]
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.failure_threshold = bhvr_
                                .config
                                .get_input("max_failures")?;
                            let children_count = bhvr_.children.len();
                            if (children_count as i32) < self_.failure_threshold {
                                return Err(
                                    BehaviorError::NodeStructure(
                                        "Number of children is less than the threshold. Can never fail."
                                            .to_string(),
                                    ),
                                );
                            }
                            let mut skipped_count = 0;
                            for i in 0..children_count {
                                if self_.completed_list.contains(&i) {
                                    continue;
                                }
                                let status = bhvr_.children[i].execute_tick().await?;
                                match status {
                                    BehaviorStatus::Success => {
                                        self_.completed_list.insert(i);
                                    }
                                    BehaviorStatus::Failure => {
                                        self_.completed_list.insert(i);
                                        self_.failure_count += 1;
                                    }
                                    BehaviorStatus::Skipped => skipped_count += 1,
                                    BehaviorStatus::Running => {}
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "ParallelAllNode".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                }
                            }
                            if skipped_count == children_count {
                                return Ok(BehaviorStatus::Skipped);
                            }
                            if skipped_count + self_.completed_list.len()
                                >= children_count
                            {
                                bhvr_.reset_children().await;
                                self_.completed_list.clear();
                                let status = if self_.failure_count
                                    >= self_.failure_threshold(bhvr_.children.len() as i32)
                                {
                                    BehaviorStatus::Failure
                                } else {
                                    BehaviorStatus::Success
                                };
                                self_.failure_count = 0;
                                return Ok(status);
                            }
                            Ok(BehaviorStatus::Running)
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let mut port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            port_info.set_default(1);
                            ("max_failures", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_children().await;
                        }
                    })
                }
            }
        }
        pub use parallel_all::*;
        mod sequence {
            #![allow(clippy::module_name_repetitions)]
            //! Built in sequence node of `DiMAS`
            #[doc(hidden)]
            extern crate alloc;
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The Sequence is used to tick children in an ordered sequence.
            /// - If any child returns RUNNING, previous children will NOT be ticked again.
            /// - If all the children return SUCCESS, this node returns SUCCESS.
            /// - If a child returns RUNNING, this node returns RUNNING.
            ///   Loop is NOT restarted, the same running child will be ticked again.
            /// - If a child returns FAILURE, stop the loop and return FAILURE.
            pub struct Sequence {
                child_idx: usize,
                all_skipped: bool,
            }
            impl Sequence {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        child_idx: 0,
                        all_skipped: false,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Sequence"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Sequence {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            if bhvr_.status == BehaviorStatus::Idle {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            while self_.child_idx < bhvr_.children.len() {
                                let cur_child = &mut bhvr_.children[self_.child_idx];
                                let _prev_status = cur_child.status();
                                let child_status = cur_child.execute_tick().await?;
                                self_.all_skipped
                                    &= child_status == BehaviorStatus::Skipped;
                                match &child_status {
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure => {
                                        bhvr_.reset_children().await;
                                        self_.child_idx = 0;
                                        return Ok(BehaviorStatus::Failure);
                                    }
                                    BehaviorStatus::Success | BehaviorStatus::Skipped => {
                                        self_.child_idx += 1;
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "SequenceNode".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                };
                            }
                            if self_.child_idx == bhvr_.children.len() {
                                bhvr_.reset_children().await;
                                self_.child_idx = 0;
                            }
                            Ok(BehaviorStatus::Success)
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.child_idx = 0;
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use sequence::*;
        mod sequence_with_memory {
            #![allow(clippy::module_name_repetitions)]
            //! Built in sequence-star node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The SequenceWithMemory is used to tick children in an ordered sequence.
            /// If any child returns RUNNING, previous children are not ticked again.
            ///
            /// - If all the children return SUCCESS, this node returns SUCCESS.
            ///
            /// - If a child returns RUNNING, this node returns RUNNING.
            ///   Loop is NOT restarted, the same running child will be ticked again.
            ///
            /// - If a child returns FAILURE, stop the loop and return FAILURE.
            ///   Loop is NOT restarted, the same running child will be ticked again.
            pub struct SequenceWithMemory {
                child_idx: usize,
                all_skipped: bool,
            }
            impl SequenceWithMemory {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        child_idx: 0,
                        all_skipped: false,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("SequenceWithMemory"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl SequenceWithMemory {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            if bhvr_.status == BehaviorStatus::Idle {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            while self_.child_idx < bhvr_.children.len() {
                                let cur_child = &mut bhvr_.children[self_.child_idx];
                                let _prev_status = cur_child.status();
                                let child_status = cur_child.execute_tick().await?;
                                self_.all_skipped
                                    &= child_status == BehaviorStatus::Skipped;
                                match &child_status {
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure => {
                                        bhvr_.halt_children(self_.child_idx).await?;
                                        return Ok(BehaviorStatus::Failure);
                                    }
                                    BehaviorStatus::Success | BehaviorStatus::Skipped => {
                                        self_.child_idx += 1;
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "SequenceStarNode".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                };
                            }
                            if self_.child_idx == bhvr_.children.len() {
                                bhvr_.reset_children().await;
                                self_.child_idx = 0;
                            }
                            if self_.all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Failure)
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.child_idx = 0;
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use sequence_with_memory::*;
        mod reactive_sequence {
            #![allow(clippy::module_name_repetitions)]
            //! Built in reactive-sequence node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The ReactiveSequence is similar to a ParallelNode.
            /// All the children are ticked from first to last:
            ///
            /// - If a child returns RUNNING, halt the remaining siblings in the sequence and return RUNNING.
            /// - If a child returns SUCCESS, tick the next sibling.
            /// - If a child returns FAILURE, stop and return FAILURE.
            ///
            /// If all the children return SUCCESS, this node returns SUCCESS.
            ///
            /// IMPORTANT: to work properly, this node should not have more than a single
            ///            asynchronous child.
            pub struct ReactiveSequence {
                running_child: i32,
            }
            impl ReactiveSequence {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self { running_child: -1 };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("ReactiveSequence"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl ReactiveSequence {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_possible_wrap)]
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            let mut all_skipped = true;
                            bhvr_.status = BehaviorStatus::Running;
                            for counter in 0..bhvr_.children.len() {
                                let child = &mut bhvr_.children[counter];
                                let child_status = child.execute_tick().await?;
                                all_skipped &= child_status == BehaviorStatus::Skipped;
                                match child_status {
                                    BehaviorStatus::Running => {
                                        for i in 0..counter {
                                            bhvr_.children[i].halt().await;
                                        }
                                        if self_.running_child == -1 {
                                            self_.running_child = counter as i32;
                                        } else if self_.running_child != counter as i32 {
                                            return Err(
                                                BehaviorError::NodeStructure(
                                                    "[ReactiveSequence]: Only a single child can return Running."
                                                        .to_string(),
                                                ),
                                            );
                                        }
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure => {
                                        bhvr_.reset_children().await;
                                        return Ok(BehaviorStatus::Failure);
                                    }
                                    BehaviorStatus::Success => {}
                                    BehaviorStatus::Skipped => {
                                        bhvr_.children[counter].halt().await;
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "ReactiveSequenceNode".into(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                }
                            }
                            bhvr_.reset_children().await;
                            if all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Success)
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use reactive_sequence::*;
        mod while_do_else {
            #![allow(clippy::module_name_repetitions)]
            //! Built in while-do-else node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// WhileDoElse must have exactly 2 or 3 children.
            /// It is a REACTIVE node of IfThenElseNode.
            ///
            /// The first child is the "statement" that is executed at each tick
            ///
            /// If result is SUCCESS, the second child is executed.
            ///
            /// If result is FAILURE, the third child is executed.
            ///
            /// If the 2nd or 3d child is RUNNING and the statement changes,
            /// the RUNNING child will be stopped before starting the sibling.
            pub struct WhileDoElse {}
            impl WhileDoElse {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("WhileDoElse"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncControl,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl WhileDoElse {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            let children_count = bhvr_.children.len();
                            if !(2..=3).contains(&children_count) {
                                return Err(
                                    BehaviorError::NodeStructure(
                                        "IfThenElseNode must have either 2 or 3 children."
                                            .to_string(),
                                    ),
                                );
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            let condition_status = bhvr_
                                .children[0]
                                .execute_tick()
                                .await?;
                            if match condition_status {
                                BehaviorStatus::Running => true,
                                _ => false,
                            } {
                                return Ok(BehaviorStatus::Running);
                            }
                            let mut status = BehaviorStatus::Idle;
                            match condition_status {
                                BehaviorStatus::Success => {
                                    if children_count == 3 {
                                        bhvr_.halt_child_idx(2).await?;
                                    }
                                    status = bhvr_.children[1].execute_tick().await?;
                                }
                                BehaviorStatus::Failure => {
                                    match children_count {
                                        3 => {
                                            bhvr_.halt_child_idx(1).await?;
                                            status = bhvr_.children[2].execute_tick().await?;
                                        }
                                        2 => {
                                            status = BehaviorStatus::Failure;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match status {
                                BehaviorStatus::Running => Ok(BehaviorStatus::Running),
                                status => {
                                    bhvr_.reset_children().await;
                                    Ok(status)
                                }
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_children().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use while_do_else::*;
    }
    pub mod decorator {
        //! Built in decorator nodes of `DiMAS`
        mod force_failure {
            #![allow(clippy::module_name_repetitions)]
            //! Built in force-failure node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The ForceFailureNode returns always Failure or Running
            pub struct ForceFailure {}
            impl ForceFailure {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("ForceFailure"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl ForceFailure {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.set_status(BehaviorStatus::Running);
                            let child_status = bhvr_
                                .child()
                                .unwrap_or_else(|| ::core::panicking::panic(
                                    "not yet implemented",
                                ))
                                .execute_tick()
                                .await?;
                            if child_status.is_completed() {
                                bhvr_.reset_child().await;
                                return Ok(BehaviorStatus::Failure);
                            }
                            Ok(child_status)
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_child().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use force_failure::*;
        mod force_success {
            #![allow(clippy::module_name_repetitions)]
            #![allow(clippy::unwrap_used)]
            //! Built in force-success node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The ForceSuccessNode returns always Success or Running
            pub struct ForceSuccess {}
            impl ForceSuccess {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("ForceSuccess"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl ForceSuccess {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.set_status(BehaviorStatus::Running);
                            let child_status = bhvr_
                                .child()
                                .unwrap_or_else(|| ::core::panicking::panic(
                                    "not yet implemented",
                                ))
                                .execute_tick()
                                .await?;
                            if child_status.is_completed() {
                                bhvr_.reset_child().await;
                                return Ok(BehaviorStatus::Success);
                            }
                            Ok(child_status)
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_child().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use force_success::*;
        mod inverter {
            #![allow(clippy::module_name_repetitions)]
            //! Built in inverter node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The InverterNode returns Failure on Success, and Success on Failure
            pub struct Inverter {}
            impl Inverter {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Inverter"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Inverter {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.set_status(BehaviorStatus::Running);
                            let child_status = bhvr_
                                .child()
                                .unwrap_or_else(|| ::core::panicking::panic(
                                    "not yet implemented",
                                ))
                                .execute_tick()
                                .await?;
                            match child_status {
                                BehaviorStatus::Success => {
                                    bhvr_.reset_child().await;
                                    Ok(BehaviorStatus::Failure)
                                }
                                BehaviorStatus::Failure => {
                                    bhvr_.reset_child().await;
                                    Ok(BehaviorStatus::Success)
                                }
                                status @ (BehaviorStatus::Running
                                | BehaviorStatus::Skipped) => Ok(status),
                                BehaviorStatus::Idle => {
                                    Err(
                                        BehaviorError::Status(
                                            "Inverter Decorator".to_string(),
                                            "Idle".to_string(),
                                        ),
                                    )
                                }
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_child().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use inverter::*;
        mod keep_running_until_failure {
            #![allow(clippy::module_name_repetitions)]
            //! Built in keep-running-until node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_macros::behavior;
            /// The KeepRunningUntilFailureNode returns always Failure or Running
            pub struct KeepRunningUntilFailure {}
            impl KeepRunningUntilFailure {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {};
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from(
                            "KeepRunningUntilFailure",
                        ),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl KeepRunningUntilFailure {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.set_status(BehaviorStatus::Running);
                            let child_status = bhvr_
                                .child()
                                .unwrap_or_else(|| ::core::panicking::panic(
                                    "not yet implemented",
                                ))
                                .execute_tick()
                                .await?;
                            match child_status {
                                BehaviorStatus::Success => {
                                    bhvr_.reset_child().await;
                                    Ok(BehaviorStatus::Running)
                                }
                                BehaviorStatus::Failure => {
                                    bhvr_.reset_child().await;
                                    Ok(BehaviorStatus::Failure)
                                }
                                _ => Ok(BehaviorStatus::Running),
                            }
                        }
                    })
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_child().await;
                        }
                    })
                }
                fn _ports() -> ::dimas_core::port::PortList {
                    ::dimas_core::port::PortList::new()
                }
            }
        }
        pub use keep_running_until_failure::*;
        mod repeat {
            #![allow(clippy::module_name_repetitions)]
            //! Built in repeat node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            /// The Retry decorator is used to execute a child several times, as long
            /// as it succeed.
            ///
            /// To succeed, the child must return SUCCESS N times (port "num_cycles").
            ///
            /// If the child returns FAILURE, the loop is stopped and this node
            /// returns FAILURE.
            ///
            /// Example:
            ///
            /// ```xml
            /// <Repeat num_cycles="3">
            ///   <ClapYourHandsOnce/>
            /// </Repeat>
            /// ```
            pub struct Repeat {
                num_cycles: i32,
                repeat_count: usize,
                all_skipped: bool,
            }
            impl Repeat {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        num_cycles: -1,
                        repeat_count: 0,
                        all_skipped: true,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Repeat"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Repeat {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.num_cycles = bhvr_.config.get_input("num_cycles")?;
                            let mut do_loop = (self_.repeat_count as i32)
                                < self_.num_cycles || self_.num_cycles == -1;
                            if match bhvr_.status {
                                BehaviorStatus::Idle => true,
                                _ => false,
                            } {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            if do_loop {
                                let child_status = bhvr_
                                    .child()
                                    .unwrap_or_else(|| ::core::panicking::panic(
                                        "not yet implemented",
                                    ))
                                    .execute_tick()
                                    .await?;
                                self_.all_skipped
                                    &= match child_status {
                                        BehaviorStatus::Skipped => true,
                                        _ => false,
                                    };
                                match child_status {
                                    BehaviorStatus::Success => {
                                        self_.repeat_count += 1;
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Failure => {
                                        self_.repeat_count = 0;
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Failure);
                                    }
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Skipped => {
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Skipped);
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "Repeat Decorator".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                }
                            }
                            self_.repeat_count = 0;
                            if self_.all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Success)
                            }
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            ("num_cycles", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.repeat_count = 0;
                            bhvr_.reset_child().await;
                        }
                    })
                }
            }
        }
        pub use repeat::*;
        mod retry {
            #![allow(clippy::module_name_repetitions)]
            //! Built in `Retry` decorator of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            /// The `Retry` decorator is used to execute a child several times if it fails.
            ///
            /// If the child returns SUCCESS, the loop is stopped and this node
            /// returns SUCCESS.
            ///
            /// If the child returns FAILURE, this behavior will try again up to N times
            /// (N is read from port "num_attempts").
            /// If N times is not enough to succeed, this decorator will return FAILURE.
            ///
            /// In contrast to the `RetryUntilSuccessful` decorator, this decorator is reactive and uses multiple ticks.
            ///
            /// Example:
            ///
            /// ```xml
            /// <Retry num_attempts="3">
            ///     <OpenDoor/>
            /// </Retry>
            /// ```
            pub struct Retry {
                max_attempts: i32,
                try_count: usize,
                all_skipped: bool,
            }
            impl Retry {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        max_attempts: -1,
                        try_count: 0,
                        all_skipped: true,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("Retry"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl Retry {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.max_attempts = bhvr_.config.get_input("num_attempts")?;
                            let mut do_loop = (self_.try_count as i32)
                                < self_.max_attempts || self_.max_attempts == -1;
                            if match bhvr_.status {
                                BehaviorStatus::Idle => true,
                                _ => false,
                            } {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            if do_loop {
                                let child_status = bhvr_
                                    .child()
                                    .unwrap_or_else(|| ::core::panicking::panic(
                                        "not yet implemented",
                                    ))
                                    .execute_tick()
                                    .await?;
                                self_.all_skipped
                                    &= match child_status {
                                        BehaviorStatus::Skipped => true,
                                        _ => false,
                                    };
                                match child_status {
                                    BehaviorStatus::Success => {
                                        self_.try_count = 0;
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Success);
                                    }
                                    BehaviorStatus::Failure => {
                                        self_.try_count += 1;
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Skipped => {
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Skipped);
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "Retry Decorator".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                }
                            }
                            self_.try_count = 0;
                            if self_.all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Failure)
                            }
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            ("num_attempts", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.try_count = 0;
                            bhvr_.reset_child().await;
                        }
                    })
                }
            }
        }
        pub use retry::*;
        mod retry_until_successful {
            #![allow(clippy::module_name_repetitions)]
            //! Built in `RetryUntilSuccessful` decorator of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::error::BehaviorError;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            /// The `RetryUntilSuccessful` decorator is used to execute a child several times if it fails.
            ///
            /// If the child returns SUCCESS, the loop is stopped and this node
            /// returns SUCCESS.
            ///
            /// If the child returns FAILURE, this decorator will try again up to N times
            /// (N is read from port "num_attempts").
            ///
            /// In contrast to the `Retry` decorator, this decorator is non-reactive and does all attempts within 1 tick.
            ///
            /// Example:
            ///
            /// ```xml
            /// <RetryUntilSuccessful num_attempts="3">
            ///     <OpenDoor/>
            /// </RetryUntilSuccessful>
            /// ```
            pub struct RetryUntilSuccessful {
                max_attempts: i32,
                try_count: usize,
                all_skipped: bool,
            }
            impl RetryUntilSuccessful {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        max_attempts: -1,
                        try_count: 0,
                        all_skipped: true,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("RetryUntilSuccessful"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl RetryUntilSuccessful {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.max_attempts = bhvr_.config.get_input("num_attempts")?;
                            let mut do_loop = (self_.try_count as i32)
                                < self_.max_attempts || self_.max_attempts == -1;
                            if match bhvr_.status {
                                BehaviorStatus::Idle => true,
                                _ => false,
                            } {
                                self_.all_skipped = true;
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            while do_loop {
                                let child_status = bhvr_
                                    .child()
                                    .unwrap_or_else(|| ::core::panicking::panic(
                                        "not yet implemented",
                                    ))
                                    .execute_tick()
                                    .await?;
                                self_.all_skipped
                                    &= match child_status {
                                        BehaviorStatus::Skipped => true,
                                        _ => false,
                                    };
                                match child_status {
                                    BehaviorStatus::Success => {
                                        self_.try_count = 0;
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Success);
                                    }
                                    BehaviorStatus::Failure => {
                                        self_.try_count += 1;
                                        do_loop = (self_.try_count as i32) < self_.max_attempts
                                            || self_.max_attempts == -1;
                                        bhvr_.reset_child().await;
                                    }
                                    BehaviorStatus::Running => {
                                        return Ok(BehaviorStatus::Running);
                                    }
                                    BehaviorStatus::Skipped => {
                                        bhvr_.reset_child().await;
                                        return Ok(BehaviorStatus::Skipped);
                                    }
                                    BehaviorStatus::Idle => {
                                        return Err(
                                            BehaviorError::Status(
                                                "RetryUntilSuccesssful Decorator".to_string(),
                                                "Idle".to_string(),
                                            ),
                                        );
                                    }
                                }
                            }
                            self_.try_count = 0;
                            if self_.all_skipped {
                                Ok(BehaviorStatus::Skipped)
                            } else {
                                Ok(BehaviorStatus::Failure)
                            }
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            ("num_attempts", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            self_.try_count = 0;
                            bhvr_.reset_child().await;
                        }
                    })
                }
            }
        }
        pub use retry_until_successful::*;
        mod run_once {
            #![allow(clippy::module_name_repetitions)]
            //! Built in run-once node of `DiMAS`
            use alloc::string::ToString;
            use dimas_core::behavior::{BehaviorResult, BehaviorStatus};
            use dimas_core::port::PortList;
            use dimas_core::{define_ports, input_port};
            use dimas_macros::behavior;
            /// The RunOnceNode is used when you want to execute the child
            /// only once.
            /// If the child is asynchronous, we will tick until either SUCCESS or FAILURE is
            /// returned.
            ///
            /// After that first execution, you can set value of the port "then_skip" to:
            ///
            /// - if TRUE (default), the node will be skipped in the future.
            /// - if FALSE, return synchronously the same status returned by the child, forever.
            pub struct RunOnce {
                already_ticked: bool,
                returned_status: BehaviorStatus,
            }
            impl RunOnce {
                /// generated behavior creation function
                pub fn create_behavior(
                    name: impl AsRef<str>,
                    config: ::dimas_core::behavior::BehaviorConfig,
                ) -> ::dimas_core::behavior::Behavior {
                    let ctx = Self {
                        already_ticked: false,
                        returned_status: BehaviorStatus::Idle,
                    };
                    let bhvr_data = ::dimas_core::behavior::BehaviorData {
                        name: name.as_ref().to_string(),
                        type_str: ::alloc::string::String::from("RunOnce"),
                        bhvr_type: ::dimas_core::behavior::BehaviorType::SyncDecorator,
                        bhvr_category: ::dimas_core::behavior::BehaviorCategory::Condition,
                        config,
                        status: ::dimas_core::behavior::BehaviorStatus::Idle,
                        children: ::alloc::vec::Vec::new(),
                        ports_fn: Self::_ports,
                    };
                    ::dimas_core::behavior::Behavior {
                        data: bhvr_data,
                        context: ::alloc::boxed::Box::new(ctx),
                        tick_fn: Self::_tick,
                        start_fn: Self::_tick,
                        halt_fn: Self::_halt,
                    }
                }
            }
            impl RunOnce {
                fn _tick<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, BehaviorResult> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            let skip = bhvr_.config.get_input("then_skip")?;
                            if self_.already_ticked {
                                return if skip {
                                    Ok(BehaviorStatus::Skipped)
                                } else {
                                    Ok(self_.returned_status.clone())
                                };
                            }
                            bhvr_.status = BehaviorStatus::Running;
                            let status = bhvr_
                                .child()
                                .unwrap_or_else(|| ::core::panicking::panic(
                                    "not yet implemented",
                                ))
                                .execute_tick()
                                .await?;
                            if status.is_completed() {
                                self_.already_ticked = true;
                                self_.returned_status = status;
                                bhvr_.reset_child().await;
                            }
                            Ok(status)
                        }
                    })
                }
                fn _ports() -> PortList {
                    {
                        let mut ports = ::dimas_core::port::PortList::new();
                        let (name, port_info) = {
                            let mut port_info = ::dimas_core::port::Port::new(
                                ::dimas_core::port::PortDirection::Input,
                            );
                            port_info.set_default(true);
                            ("then_skip", port_info)
                        };
                        ports.insert(::alloc::string::String::from(name), port_info);
                        ports
                    }
                }
                fn _halt<'a>(
                    bhvr_: &'a mut ::dimas_core::behavior::BehaviorData,
                    ctx: &'a mut ::alloc::boxed::Box<
                        dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
                    >,
                ) -> ::futures::future::BoxFuture<'a, ()> {
                    ::alloc::boxed::Box::pin(async move {
                        let mut self_ = ctx.downcast_mut::<Self>().unwrap();
                        {
                            bhvr_.reset_child().await;
                        }
                    })
                }
            }
        }
        pub use run_once::*;
    }
}
pub mod factory {
    //! [`BehaviorTree`] factory of `DiMAS`
    #[doc(hidden)]
    extern crate alloc;
    mod error {
        #![allow(unused)]
        //! `BTFactory` errors
        #[doc(hidden)]
        extern crate alloc;
        use core::str::ParseBoolError;
        use alloc::{
            string::{FromUtf8Error, String},
            vec::Vec,
        };
        use evalexpr::{DefaultNumericTypes, EvalexprError};
        use thiserror::Error;
        /// `dimas` error type
        pub enum Error {
            /// @TODO:
            #[error("'BTCPP_format' must be '4'")]
            BtCppFormat,
            /// @TODO:
            #[error("children are not allowed for behavior category [{0}]")]
            ChildrenNotAllowed(String),
            /// @TODO:
            #[error("{0}")]
            DimasCoreBehavior(#[from] dimas_core::behavior::error::BehaviorError),
            /// @TODO:
            #[error("decorator [{0}] must have 1 child")]
            DecoratorChildren(String),
            /// @TODO:
            #[error("element [{0}] is not supported")]
            ElementNotSupported(String),
            /// @TODO:
            #[error("loop in tree detected: [{0}] -> [{1}]")]
            LoopDetected(String, String),
            /// @TODO:
            #[error(
                "attribute 'main_tree_to_execute' not allowed in subtree definition"
            )]
            MainTreeNotAllowed,
            /// @TODO:
            #[error("missing attribute [{0}]")]
            MissingAttribute(String),
            /// @TODO:
            #[error("missing attribute 'ID' in tag [{0}]")]
            MissingId(String),
            /// @TODO:
            #[error("missing end tag for [{0}]")]
            MissingEndTag(String),
            /// @TODO:
            #[error("missing tag [{0}]")]
            MissingTag(String),
            /// @TODO:
            #[error("no main tree provided")]
            NoMainTree,
            /// @TODO:
            #[error("no behavior content found")]
            NoTreeContent,
            /// @TODO:
            #[error("no 'main_tree_to_execute' provided")]
            NoTreeToExecute,
            /// @TODO:
            #[error("{0}")]
            ParseBool(#[from] ParseBoolError),
            /// @TODO:
            #[error("Error parsing expression in port value: {0}")]
            PortExpressionInvalid(#[from] EvalexprError<DefaultNumericTypes>),
            /// @TODO:
            #[error("invalid type [{0}] for variable [{1}]")]
            PortExpressionInvalidType(String, String),
            /// @TODO:
            #[error("variable in blackboard pointer [{0}] has no type")]
            PortExpressionMissingType(String),
            /// @TODO:
            #[error("port name [{0}] does not match nodes [{1}] port list: {2:?}")]
            PortInvalid(String, String, Vec<String>),
            /// @TODO:
            #[error("port [{0}] in [{1}]has no default value")]
            PortWithoutDefault(String, String),
            /// @TODO:
            #[error("root element must be named 'root'")]
            RootName,
            /// @TODO:
            #[error("{0}")]
            RoXmlTree(#[from] roxmltree::Error),
            /// @TODO:
            #[error("unkown behavior [{0}]")]
            UnknownBehavior(String),
            /// @TODO:
            #[error("unkown element [{0}]")]
            UnknownElement(String),
            /// @TODO:
            #[error("processing instructions are not supported")]
            UnkownProcessingInstruction,
            /// @TODO:
            #[error("unexpected [{0}] in file [{1}] at line [{2}]")]
            Unexpected(String, String, u32),
            /// @TODO:
            #[error("Error parsing UTF8: {0}")]
            Utf8(#[from] FromUtf8Error),
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::thiserror::__private::Error for Error {
            fn source(
                &self,
            ) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
                use ::thiserror::__private::AsDynError as _;
                #[allow(deprecated)]
                match self {
                    Error::BtCppFormat { .. } => ::core::option::Option::None,
                    Error::ChildrenNotAllowed { .. } => ::core::option::Option::None,
                    Error::DimasCoreBehavior { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::DecoratorChildren { .. } => ::core::option::Option::None,
                    Error::ElementNotSupported { .. } => ::core::option::Option::None,
                    Error::LoopDetected { .. } => ::core::option::Option::None,
                    Error::MainTreeNotAllowed { .. } => ::core::option::Option::None,
                    Error::MissingAttribute { .. } => ::core::option::Option::None,
                    Error::MissingId { .. } => ::core::option::Option::None,
                    Error::MissingEndTag { .. } => ::core::option::Option::None,
                    Error::MissingTag { .. } => ::core::option::Option::None,
                    Error::NoMainTree { .. } => ::core::option::Option::None,
                    Error::NoTreeContent { .. } => ::core::option::Option::None,
                    Error::NoTreeToExecute { .. } => ::core::option::Option::None,
                    Error::ParseBool { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::PortExpressionInvalid { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::PortExpressionInvalidType { .. } => {
                        ::core::option::Option::None
                    }
                    Error::PortExpressionMissingType { .. } => {
                        ::core::option::Option::None
                    }
                    Error::PortInvalid { .. } => ::core::option::Option::None,
                    Error::PortWithoutDefault { .. } => ::core::option::Option::None,
                    Error::RootName { .. } => ::core::option::Option::None,
                    Error::RoXmlTree { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::UnknownBehavior { .. } => ::core::option::Option::None,
                    Error::UnknownElement { .. } => ::core::option::Option::None,
                    Error::UnkownProcessingInstruction { .. } => {
                        ::core::option::Option::None
                    }
                    Error::Unexpected { .. } => ::core::option::Option::None,
                    Error::Utf8 { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::core::fmt::Display for Error {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use ::thiserror::__private::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::BtCppFormat {} => {
                        __formatter.write_str("'BTCPP_format' must be '4'")
                    }
                    Error::ChildrenNotAllowed(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "children are not allowed for behavior category [{0}]",
                                            __display0,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::DimasCoreBehavior(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter.write_fmt(format_args!("{0}", __display0))
                            }
                        }
                    }
                    Error::DecoratorChildren(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "decorator [{0}] must have 1 child",
                                            __display0,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::ElementNotSupported(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("element [{0}] is not supported", __display0),
                                    )
                            }
                        }
                    }
                    Error::LoopDetected(_0, _1) => {
                        match (_0.as_display(), _1.as_display()) {
                            (__display0, __display1) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "loop in tree detected: [{0}] -> [{1}]",
                                            __display0,
                                            __display1,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::MainTreeNotAllowed {} => {
                        __formatter
                            .write_str(
                                "attribute 'main_tree_to_execute' not allowed in subtree definition",
                            )
                    }
                    Error::MissingAttribute(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("missing attribute [{0}]", __display0),
                                    )
                            }
                        }
                    }
                    Error::MissingId(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "missing attribute \'ID\' in tag [{0}]",
                                            __display0,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::MissingEndTag(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("missing end tag for [{0}]", __display0),
                                    )
                            }
                        }
                    }
                    Error::MissingTag(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(format_args!("missing tag [{0}]", __display0))
                            }
                        }
                    }
                    Error::NoMainTree {} => {
                        __formatter.write_str("no main tree provided")
                    }
                    Error::NoTreeContent {} => {
                        __formatter.write_str("no behavior content found")
                    }
                    Error::NoTreeToExecute {} => {
                        __formatter.write_str("no 'main_tree_to_execute' provided")
                    }
                    Error::ParseBool(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter.write_fmt(format_args!("{0}", __display0))
                            }
                        }
                    }
                    Error::PortExpressionInvalid(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "Error parsing expression in port value: {0}",
                                            __display0,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::PortExpressionInvalidType(_0, _1) => {
                        match (_0.as_display(), _1.as_display()) {
                            (__display0, __display1) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "invalid type [{0}] for variable [{1}]",
                                            __display0,
                                            __display1,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::PortExpressionMissingType(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "variable in blackboard pointer [{0}] has no type",
                                            __display0,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::PortInvalid(_0, _1, _2) => {
                        match (_0.as_display(), _1.as_display(), _2) {
                            (__display0, __display1, __field2) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "port name [{0}] does not match nodes [{1}] port list: {2:?}",
                                            __display0,
                                            __display1,
                                            __field2,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::PortWithoutDefault(_0, _1) => {
                        match (_0.as_display(), _1.as_display()) {
                            (__display0, __display1) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "port [{0}] in [{1}]has no default value",
                                            __display0,
                                            __display1,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::RootName {} => {
                        __formatter.write_str("root element must be named 'root'")
                    }
                    Error::RoXmlTree(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter.write_fmt(format_args!("{0}", __display0))
                            }
                        }
                    }
                    Error::UnknownBehavior(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("unkown behavior [{0}]", __display0),
                                    )
                            }
                        }
                    }
                    Error::UnknownElement(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(format_args!("unkown element [{0}]", __display0))
                            }
                        }
                    }
                    Error::UnkownProcessingInstruction {} => {
                        __formatter
                            .write_str("processing instructions are not supported")
                    }
                    Error::Unexpected(_0, _1, _2) => {
                        match (_0.as_display(), _1.as_display(), _2.as_display()) {
                            (__display0, __display1, __display2) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "unexpected [{0}] in file [{1}] at line [{2}]",
                                            __display0,
                                            __display1,
                                            __display2,
                                        ),
                                    )
                            }
                        }
                    }
                    Error::Utf8(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Error parsing UTF8: {0}", __display0),
                                    )
                            }
                        }
                    }
                }
            }
        }
        #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
        #[automatically_derived]
        impl ::core::convert::From<dimas_core::behavior::error::BehaviorError>
        for Error {
            fn from(source: dimas_core::behavior::error::BehaviorError) -> Self {
                Error::DimasCoreBehavior {
                    0: source,
                }
            }
        }
        #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
        #[automatically_derived]
        impl ::core::convert::From<ParseBoolError> for Error {
            fn from(source: ParseBoolError) -> Self {
                Error::ParseBool { 0: source }
            }
        }
        #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
        #[automatically_derived]
        impl ::core::convert::From<EvalexprError<DefaultNumericTypes>> for Error {
            fn from(source: EvalexprError<DefaultNumericTypes>) -> Self {
                Error::PortExpressionInvalid {
                    0: source,
                }
            }
        }
        #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
        #[automatically_derived]
        impl ::core::convert::From<roxmltree::Error> for Error {
            fn from(source: roxmltree::Error) -> Self {
                Error::RoXmlTree { 0: source }
            }
        }
        #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
        #[automatically_derived]
        impl ::core::convert::From<FromUtf8Error> for Error {
            fn from(source: FromUtf8Error) -> Self {
                Error::Utf8 { 0: source }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::BtCppFormat => {
                        ::core::fmt::Formatter::write_str(f, "BtCppFormat")
                    }
                    Error::ChildrenNotAllowed(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ChildrenNotAllowed",
                            &__self_0,
                        )
                    }
                    Error::DimasCoreBehavior(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "DimasCoreBehavior",
                            &__self_0,
                        )
                    }
                    Error::DecoratorChildren(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "DecoratorChildren",
                            &__self_0,
                        )
                    }
                    Error::ElementNotSupported(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ElementNotSupported",
                            &__self_0,
                        )
                    }
                    Error::LoopDetected(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "LoopDetected",
                            __self_0,
                            &__self_1,
                        )
                    }
                    Error::MainTreeNotAllowed => {
                        ::core::fmt::Formatter::write_str(f, "MainTreeNotAllowed")
                    }
                    Error::MissingAttribute(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MissingAttribute",
                            &__self_0,
                        )
                    }
                    Error::MissingId(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MissingId",
                            &__self_0,
                        )
                    }
                    Error::MissingEndTag(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MissingEndTag",
                            &__self_0,
                        )
                    }
                    Error::MissingTag(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MissingTag",
                            &__self_0,
                        )
                    }
                    Error::NoMainTree => {
                        ::core::fmt::Formatter::write_str(f, "NoMainTree")
                    }
                    Error::NoTreeContent => {
                        ::core::fmt::Formatter::write_str(f, "NoTreeContent")
                    }
                    Error::NoTreeToExecute => {
                        ::core::fmt::Formatter::write_str(f, "NoTreeToExecute")
                    }
                    Error::ParseBool(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ParseBool",
                            &__self_0,
                        )
                    }
                    Error::PortExpressionInvalid(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PortExpressionInvalid",
                            &__self_0,
                        )
                    }
                    Error::PortExpressionInvalidType(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "PortExpressionInvalidType",
                            __self_0,
                            &__self_1,
                        )
                    }
                    Error::PortExpressionMissingType(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PortExpressionMissingType",
                            &__self_0,
                        )
                    }
                    Error::PortInvalid(__self_0, __self_1, __self_2) => {
                        ::core::fmt::Formatter::debug_tuple_field3_finish(
                            f,
                            "PortInvalid",
                            __self_0,
                            __self_1,
                            &__self_2,
                        )
                    }
                    Error::PortWithoutDefault(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "PortWithoutDefault",
                            __self_0,
                            &__self_1,
                        )
                    }
                    Error::RootName => ::core::fmt::Formatter::write_str(f, "RootName"),
                    Error::RoXmlTree(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "RoXmlTree",
                            &__self_0,
                        )
                    }
                    Error::UnknownBehavior(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "UnknownBehavior",
                            &__self_0,
                        )
                    }
                    Error::UnknownElement(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "UnknownElement",
                            &__self_0,
                        )
                    }
                    Error::UnkownProcessingInstruction => {
                        ::core::fmt::Formatter::write_str(
                            f,
                            "UnkownProcessingInstruction",
                        )
                    }
                    Error::Unexpected(__self_0, __self_1, __self_2) => {
                        ::core::fmt::Formatter::debug_tuple_field3_finish(
                            f,
                            "Unexpected",
                            __self_0,
                            __self_1,
                            &__self_2,
                        )
                    }
                    Error::Utf8(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Utf8",
                            &__self_0,
                        )
                    }
                }
            }
        }
    }
    #[allow(clippy::module_inception)]
    mod factory {
        //! [`BehaviorTree`] factory of `DiMAS`
        extern crate std;
        use std::dbg;
        use alloc::{
            string::{String, ToString},
            sync::Arc, vec::Vec,
        };
        use core::fmt::Debug;
        use dimas_core::{
            behavior::{tree::BehaviorTree, Behavior, BehaviorCategory, BehaviorConfig},
            blackboard::Blackboard, build_bhvr_ptr,
        };
        use hashbrown::HashMap;
        use roxmltree::{Document, Node, NodeType, ParsingOptions};
        use tracing::{instrument, Level};
        use crate::builtin::{
            control::{
                Fallback, IfThenElse, Parallel, ParallelAll, ReactiveFallback,
                ReactiveSequence, Sequence, SequenceWithMemory, WhileDoElse,
            },
            decorator::{
                ForceFailure, ForceSuccess, Inverter, KeepRunningUntilFailure, Repeat,
                Retry, RetryUntilSuccessful, RunOnce,
            },
        };
        use super::{error::Error, xml_parser::XmlParser};
        /// @TODO:
        pub type BehaviorCreateFn = dyn Fn(
            BehaviorConfig,
            Vec<Behavior>,
        ) -> Behavior + Send + Sync;
        #[allow(clippy::module_name_repetitions)]
        pub struct FactoryData {
            pub main_tree_id: Option<String>,
            pub bhvr_map: HashMap<String, (BehaviorCategory, Arc<BehaviorCreateFn>)>,
            pub tree_definitions: HashMap<String, String>,
        }
        impl FactoryData {
            #[allow(clippy::too_many_lines)]
            fn add_extensions(&mut self) {
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <ForceFailure>::create_behavior(
                            "ForceFailure",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "ForceFailure",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "ForceFailure".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <ForceSuccess>::create_behavior(
                            "ForceSuccess",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "ForceSuccess",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "ForceSuccess".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <IfThenElse>::create_behavior(
                            "IfThenElse",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "IfThenElse",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "IfThenElse".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Inverter>::create_behavior(
                            "Inverter",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Inverter",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "Inverter".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <KeepRunningUntilFailure>::create_behavior(
                            "KeepRunningUntilFailure",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "KeepRunningUntilFailure",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "KeepRunningUntilFailure".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <ParallelAll>::create_behavior(
                            "ParallelAll",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "ParallelAll",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "ParallelAll".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <ReactiveFallback>::create_behavior(
                            "ReactiveFallback",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "ReactiveFallback",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "ReactiveFallback".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <ReactiveSequence>::create_behavior(
                            "ReactiveSequence",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "ReactiveSequence",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "ReactiveSequence".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Repeat>::create_behavior("Repeat", config);
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Repeat",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "Repeat".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Retry>::create_behavior("Retry", config);
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Retry",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "Retry".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <RetryUntilSuccessful>::create_behavior(
                            "RetryUntilSuccessful",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "RetryUntilSuccessful",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "RetryUntilSuccessful".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <RunOnce>::create_behavior("RunOnce", config);
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "RunOnce",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "RunOnce".into(),
                        (BehaviorCategory::Decorator, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <SequenceWithMemory>::create_behavior(
                            "SequenceWithMemory",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "SequenceWithMemory",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "SequenceWithMemory".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <WhileDoElse>::create_behavior(
                            "WhileDoElse",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "WhileDoElse",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                self.bhvr_map
                    .insert(
                        "WhileDoElse".into(),
                        (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                    );
            }
            fn create_fundamentals() -> HashMap<
                String,
                (BehaviorCategory, Arc<BehaviorCreateFn>),
            > {
                let mut map: HashMap<
                    String,
                    (BehaviorCategory, Arc<BehaviorCreateFn>),
                > = HashMap::new();
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Fallback>::create_behavior(
                            "Fallback",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Fallback",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                map.insert(
                    "Fallback".into(),
                    (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Parallel>::create_behavior(
                            "Parallel",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Parallel",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                map.insert(
                    "Parallel".into(),
                    (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                );
                let bhvr_fn = move |
                    config: BehaviorConfig,
                    children: Vec<Behavior>,
                | -> Behavior {
                    let mut bhvr = {
                        let mut behavior = <Sequence>::create_behavior(
                            "Sequence",
                            config,
                        );
                        let manifest = ::dimas_core::behavior::BehaviorManifest::new(
                            behavior.bhvr_category(),
                            "Sequence",
                            behavior.provided_ports(),
                            "",
                        );
                        behavior
                            .config_mut()
                            .set_manifest(::alloc::sync::Arc::new(manifest));
                        behavior
                    };
                    bhvr.data.children = children;
                    bhvr
                };
                map.insert(
                    "Sequence".into(),
                    (BehaviorCategory::Control, Arc::new(bhvr_fn)),
                );
                map
            }
            pub fn register_behavior<F>(
                &mut self,
                name: impl Into<String>,
                bhvr_fn: F,
                bhvr_type: BehaviorCategory,
            )
            where
                F: Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync + 'static,
            {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "register_behavior",
                                        "dimas_config::factory::factory",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/factory.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(249u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::factory",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: () = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            self.bhvr_map
                                .insert(name.into(), (bhvr_type, Arc::new(bhvr_fn)));
                        }
                    }
                }
            }
        }
        impl Default for FactoryData {
            fn default() -> Self {
                Self {
                    main_tree_id: None,
                    bhvr_map: Self::create_fundamentals(),
                    tree_definitions: HashMap::new(),
                }
            }
        }
        /// @TODO:
        #[allow(clippy::module_name_repetitions)]
        pub struct BTFactory {
            blackboard: Blackboard,
            data: FactoryData,
        }
        impl BTFactory {
            /// Create an empty behavior factory using the given [`Blackboard`].
            #[must_use]
            pub fn extended() -> Self {
                let mut data = FactoryData::default();
                data.add_extensions();
                Self::new(Blackboard::default(), data)
            }
            /// Constructor
            #[must_use]
            pub const fn new(blackboard: Blackboard, data: FactoryData) -> Self {
                Self { blackboard, data }
            }
            /// Create an empty behavior factory using the given [`Blackboard`].
            #[must_use]
            pub fn with_blackboard(blackboard: Blackboard) -> Self {
                Self {
                    blackboard,
                    data: FactoryData::default(),
                }
            }
            /// @TODO:
            pub fn add_extensions(&mut self) {
                self.data.add_extensions();
            }
            /// @TODO:
            #[must_use]
            pub const fn blackboard(&self) -> &Blackboard {
                &self.blackboard
            }
            /// @TODO:
            /// # Errors
            pub fn create_tree_from_xml(
                &mut self,
                xml: &str,
            ) -> Result<BehaviorTree, Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "create_tree_from_xml",
                                        "dimas_config::factory::factory",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/factory.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(319u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::factory",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<
                                BehaviorTree,
                                Error,
                            > = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            let doc = Document::parse(xml)?;
                            let root = doc.root_element();
                            if root.tag_name().name() != "root" {
                                return Err(Error::RootName);
                            }
                            if let Some(format) = root.attribute("BTCPP_format") {
                                if format != "4" {
                                    return Err(Error::BtCppFormat);
                                }
                            }
                            let xml = Self::shrink_xml(xml);
                            let root_bhvr = XmlParser::parse_main_xml(
                                &self.blackboard,
                                &mut self.data,
                                &xml,
                            )?;
                            Ok(BehaviorTree::new(root_bhvr))
                        }
                    }
                }
            }
            /// @TODO:
            #[inline]
            pub fn register_behavior<F>(
                &mut self,
                name: impl Into<String>,
                bhvr_fn: F,
                bhvr_type: BehaviorCategory,
            )
            where
                F: Fn(BehaviorConfig, Vec<Behavior>) -> Behavior + Send + Sync + 'static,
            {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "register_behavior",
                                        "dimas_config::factory::factory",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/factory.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(342u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::factory",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: () = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            self.data.register_behavior(name.into(), bhvr_fn, bhvr_type);
                        }
                    }
                }
            }
            /// @TODO:
            /// # Errors;
            pub fn register_subtree(&mut self, xml: &str) -> Result<(), Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "register_subtree",
                                        "dimas_config::factory::factory",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/factory.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(357u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::factory",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<(), Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            let xml = Self::shrink_xml(xml);
                            XmlParser::parse_sub_xml(
                                &self.blackboard,
                                &mut self.data,
                                &xml,
                            )
                        }
                    }
                }
            }
            /// Reduce the input XML by eliminating everything
            /// that is not information but only formatting
            fn shrink_xml(input: &str) -> String {
                let mut res = String::with_capacity(input.len());
                let mut in_whitespaces = false;
                let mut in_tag = false;
                let mut in_literal = false;
                let mut in_assignment = false;
                for char in input.chars() {
                    match char {
                        '\n' => continue,
                        '<' => {
                            in_tag = true;
                            in_whitespaces = false;
                            in_assignment = false;
                        }
                        '>' => {
                            in_tag = false;
                            in_whitespaces = false;
                            in_assignment = false;
                        }
                        '"' => {
                            in_literal = !in_literal;
                            in_whitespaces = false;
                            in_assignment = false;
                        }
                        ' ' | '\t' => {
                            if !in_tag {
                                continue;
                            }
                            if !in_literal {
                                if in_whitespaces || in_assignment {
                                    continue;
                                }
                                in_whitespaces = true;
                            }
                        }
                        '=' => {
                            if in_whitespaces {
                                res.pop();
                            }
                            in_whitespaces = false;
                            in_assignment = true;
                        }
                        _ => {
                            in_whitespaces = false;
                            in_assignment = false;
                        }
                    };
                    res.push(char);
                }
                res.shrink_to_fit();
                res
            }
        }
        impl Debug for BTFactory {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("BTFactory")
                    .field("blackboard", &self.blackboard)
                    .finish_non_exhaustive()
            }
        }
        impl Default for BTFactory {
            fn default() -> Self {
                Self::new(Blackboard::default(), FactoryData::default())
            }
        }
    }
    mod xml_parser {
        //! XML parser for the [`BehaviorTree`] factory [`BTFactory`] of `DiMAS`
        use alloc::{
            borrow::ToOwned, format, string::{String, ToString},
            sync::Arc, vec::Vec,
        };
        use dimas_core::{
            behavior::{Behavior, BehaviorCategory, BehaviorConfig},
            blackboard::{Blackboard, BlackboardString},
            build_bhvr_ptr, port::{PortChecks, PortDirection, PortRemapping},
        };
        use hashbrown::HashMap;
        use roxmltree::{Attributes, Document, Node, NodeType, ParsingOptions};
        use tracing::{event, instrument, Level};
        use super::{error::Error, factory::{BehaviorCreateFn, FactoryData}};
        enum CreateBehaviorResult {
            Behavior(Behavior),
            Continue,
            End,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateBehaviorResult {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CreateBehaviorResult::Behavior(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Behavior",
                            &__self_0,
                        )
                    }
                    CreateBehaviorResult::Continue => {
                        ::core::fmt::Formatter::write_str(f, "Continue")
                    }
                    CreateBehaviorResult::End => {
                        ::core::fmt::Formatter::write_str(f, "End")
                    }
                }
            }
        }
        pub trait AttrsToMap {
            fn to_map(self) -> Result<HashMap<String, String>, Error>;
        }
        impl AttrsToMap for Attributes<'_, '_> {
            fn to_map(self) -> Result<HashMap<String, String>, Error> {
                let mut map = HashMap::new();
                for attr in self {
                    let name = attr.name().into();
                    let value = attr.value().to_string();
                    map.insert(name, value);
                }
                Ok(map)
            }
        }
        pub struct XmlParser {}
        impl XmlParser {
            /// @TODO:
            /// # Errors
            fn add_ports(
                bhvr_ptr: &mut Behavior,
                bhvr_name: &str,
                attributes: Attributes,
            ) -> Result<(), Error> {
                let config = bhvr_ptr.config_mut();
                let manifest = config.manifest()?;
                let mut remap = PortRemapping::new();
                for (port_name, port_value) in attributes.to_map()? {
                    remap.insert(port_name, port_value);
                }
                for port_name in remap.keys() {
                    if !manifest.ports.contains_key(port_name) {
                        return Err(
                            Error::PortInvalid(
                                port_name.clone(),
                                bhvr_name.to_owned(),
                                manifest.ports.clone().into_keys().collect(),
                            ),
                        );
                    }
                }
                for (remap_name, remap_val) in remap {
                    if let Some(port) = manifest.ports.get(&remap_name) {
                        if port.parse_expr() {
                            let expr = evalexpr::build_operator_tree::<
                                evalexpr::DefaultNumericTypes,
                            >(&remap_val)?;
                            for key in expr.iter_variable_identifiers() {
                                if key.starts_with('{') && key.ends_with('}') {
                                    let inner_key = &key[1..(key.len() - 1)];
                                    let (name, var_type) = inner_key
                                        .split_once(':')
                                        .ok_or_else(|| {
                                            Error::PortExpressionMissingType(inner_key.to_owned())
                                        })?;
                                    match var_type {
                                        "int" | "float" | "str" | "bool" => {}
                                        _ => {
                                            return Err(
                                                Error::PortExpressionInvalidType(
                                                    var_type.to_owned(),
                                                    name.to_owned(),
                                                ),
                                            );
                                        }
                                    };
                                }
                            }
                        }
                        config.add_port(port.direction(), remap_name, remap_val);
                    }
                }
                for (port_name, port_info) in &manifest.ports {
                    let direction = port_info.direction();
                    if !match direction {
                        PortDirection::Output => true,
                        _ => false,
                    } && !config.has_port(direction, port_name)
                        && port_info.default_value().is_some()
                    {
                        let value = port_info
                            .default_value_str()
                            .ok_or_else(|| {
                                Error::PortWithoutDefault(
                                    port_name.clone(),
                                    config.path.clone(),
                                )
                            })?;
                        config.add_port(&PortDirection::Input, port_name.clone(), value);
                    }
                }
                Ok(())
            }
            fn find_in_map(
                element: Node,
                data: &FactoryData,
            ) -> Result<(BehaviorCategory, Arc<BehaviorCreateFn>), Error> {
                let bhvr_name = element.tag_name().name();
                if let Some(id) = element.attribute("ID") {
                    Ok(
                        data
                            .bhvr_map
                            .get(id)
                            .ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
                            .clone(),
                    )
                } else {
                    Ok(
                        data
                            .bhvr_map
                            .get(bhvr_name)
                            .ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
                            .clone(),
                    )
                }
            }
            /// @TODO:
            /// # Errors
            fn build_child(
                element: Node,
                data: &mut FactoryData,
                blackboard: &Blackboard,
                tree_name: &str,
                path: &str,
            ) -> Result<Behavior, Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "build_child",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(166u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<Behavior, Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:174",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(174u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("build_child") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let res = Self::find_in_map(element, data);
                            let Ok((bhvr_category, bhvr_fn)) = res else {
                                return Self::build_subtree(element, data, blackboard, path);
                            };
                            let bhvr_name = element.tag_name().name();
                            let attributes = element.attributes();
                            let mut config = BehaviorConfig::new(
                                blackboard.clone(),
                                path.to_owned() + "->" + bhvr_name,
                            );
                            let bhvr = match bhvr_category {
                                BehaviorCategory::Action | BehaviorCategory::Condition => {
                                    if element.has_children() {
                                        return Err(
                                            Error::ChildrenNotAllowed(bhvr_category.to_string()),
                                        );
                                    }
                                    let mut behavior = bhvr_fn(config, Vec::new());
                                    Self::add_ports(&mut behavior, bhvr_name, attributes)?;
                                    behavior
                                }
                                BehaviorCategory::Control => {
                                    let children = Self::build_children(
                                        element,
                                        data,
                                        blackboard,
                                        tree_name,
                                        path,
                                    )?;
                                    let mut behavior = bhvr_fn(config, children);
                                    Self::add_ports(&mut behavior, bhvr_name, attributes)?;
                                    behavior
                                }
                                BehaviorCategory::Decorator => {
                                    let children = Self::build_children(
                                        element,
                                        data,
                                        blackboard,
                                        tree_name,
                                        path,
                                    )?;
                                    if children.len() != 1 {
                                        return Err(
                                            Error::DecoratorChildren(element.tag_name().name().into()),
                                        );
                                    }
                                    let mut behavior = bhvr_fn(config, children);
                                    Self::add_ports(&mut behavior, bhvr_name, attributes)?;
                                    behavior
                                }
                            };
                            Ok(bhvr)
                        }
                    }
                }
            }
            /// @TODO:
            /// # Errors
            fn build_children(
                element: Node,
                data: &mut FactoryData,
                blackboard: &Blackboard,
                tree_name: &str,
                path: &str,
            ) -> Result<Vec<Behavior>, Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "build_children",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(218u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<
                                Vec<Behavior>,
                                Error,
                            > = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:226",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(226u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("build_children") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let mut children: Vec<Behavior> = Vec::new();
                            for child in element.children() {
                                match child.node_type() {
                                    NodeType::Comment | NodeType::Text => {}
                                    NodeType::Root => {
                                        ::core::panicking::panic("not yet implemented")
                                    }
                                    NodeType::Element => {
                                        let behavior = Self::build_child(
                                            child,
                                            data,
                                            blackboard,
                                            tree_name,
                                            path,
                                        )?;
                                        children.push(behavior);
                                    }
                                    NodeType::PI => {
                                        return Err(Error::UnkownProcessingInstruction);
                                    }
                                }
                            }
                            Ok(children)
                        }
                    }
                }
            }
            fn build_subtree(
                element: Node,
                data: &mut FactoryData,
                blackboard: &Blackboard,
                path: &str,
            ) -> Result<Behavior, Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "build_subtree",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(246u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<Behavior, Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:253",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(253u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("build_subtree") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            if let Some(id) = element.attribute("ID") {
                                let definition = match data.tree_definitions.get(id) {
                                    Some(def) => def.to_owned(),
                                    None => return Err(Error::UnknownBehavior(id.into())),
                                };
                                let doc = Document::parse(&definition)?;
                                let root = doc.root_element();
                                let attributes = element.attributes();
                                if path.contains(id) {
                                    return Err(Error::LoopDetected(path.into(), id.into()));
                                }
                                let path = path.to_owned() + "->" + id;
                                let attributes = attributes.to_map()?;
                                let mut subtree_blackboard = Blackboard::new(blackboard);
                                for (attr, value) in attributes {
                                    if attr == "_autoremap" {
                                        let val = value.parse::<bool>()?;
                                        subtree_blackboard.enable_auto_remapping(val);
                                        continue;
                                    } else if !attr.is_allowed_port_name() {
                                        continue;
                                    }
                                    if let Some(port_name) = value.strip_bb_pointer() {
                                        subtree_blackboard
                                            .add_subtree_remapping(attr.clone(), port_name);
                                    } else {
                                        subtree_blackboard.set(attr, value.clone());
                                    }
                                }
                                Self::build_child(
                                    root,
                                    data,
                                    &subtree_blackboard,
                                    id,
                                    &path,
                                )
                            } else {
                                let bhvr_name = element.tag_name().name();
                                Err(Error::UnknownBehavior(bhvr_name.into()))
                            }
                        }
                    }
                }
            }
            /// @TODO:
            /// # Errors
            fn get_build_instructions(element: Node, id: &str) -> Result<String, Error> {
                let source = element.document().input_text();
                let pattern = ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("BehaviorTree ID=\"{0}\"", id),
                    );
                    res
                });
                let start = pattern.len() + 1
                    + source.find(&pattern).ok_or_else(|| Error::MissingId(id.into()))?;
                let end = start
                    + source[start..]
                        .find("</BehaviorTree")
                        .ok_or_else(|| Error::MissingEndTag("BehaviorTree".into()))?;
                let res = source[start..end].to_string();
                Ok(res)
            }
            /// @TODO:
            /// # Errors
            fn parse_document(
                doc: Node,
                data: &mut FactoryData,
                blackboard: &Blackboard,
            ) -> Result<(), Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "parse_document",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(321u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<(), Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:327",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(327u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("parse_document") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let mut root_behavior: Option<Behavior> = None;
                            for element in doc.children() {
                                match element.node_type() {
                                    NodeType::Comment | NodeType::Text => {}
                                    NodeType::Root => {
                                        ::core::panicking::panic("not yet implemented")
                                    }
                                    NodeType::Element => {
                                        match element.tag_name().name() {
                                            "TreeNodesModel" => {}
                                            "BehaviorTree" => {
                                                if let Some(id) = element.attribute("ID") {
                                                    let bi = Self::get_build_instructions(element, id)?;
                                                    data.tree_definitions.insert(id.into(), bi);
                                                } else {
                                                    return Err(
                                                        Error::MissingId(element.tag_name().name().into()),
                                                    );
                                                };
                                            }
                                            _ => {
                                                return Err(
                                                    Error::ElementNotSupported(element.tag_name().name().into()),
                                                );
                                            }
                                        }
                                    }
                                    NodeType::PI => {
                                        return Err(Error::UnkownProcessingInstruction);
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                }
            }
            /// @TODO:
            /// # Errors
            pub fn parse_main_xml(
                blackboard: &Blackboard,
                data: &mut FactoryData,
                xml: &str,
            ) -> Result<Behavior, Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "parse_main_xml",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(366u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<Behavior, Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:372",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(372u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("parse_main_xml") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let doc = Document::parse(xml)?;
                            let root = doc.root_element();
                            if let Some(id) = root.attribute("main_tree_to_execute") {
                                data.main_tree_id = Some(id.into());
                                Self::parse_document(root, data, blackboard)?;
                                let definition = match data.tree_definitions.get(id) {
                                    Some(def) => def.to_owned(),
                                    None => return Err(Error::UnknownBehavior(id.into())),
                                };
                                let doc = Document::parse(&definition)?;
                                let main_tree = doc.root_element();
                                Self::build_child(main_tree, data, blackboard, id, id)
                            } else {
                                Err(Error::NoTreeToExecute)
                            }
                        }
                    }
                }
            }
            /// @TODO:
            /// # Errors
            pub fn parse_sub_xml(
                blackboard: &Blackboard,
                data: &mut FactoryData,
                xml: &str,
            ) -> Result<(), Error> {
                {}
                #[allow(clippy::suspicious_else_formatting)]
                {
                    let __tracing_attr_span;
                    let __tracing_attr_guard;
                    if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        || { false }
                    {
                        __tracing_attr_span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "parse_sub_xml",
                                        "dimas_config::factory::xml_parser",
                                        Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "commons/dimas-config/src/factory/xml_parser.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(395u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "dimas_config::factory::xml_parser",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        __tracing_attr_guard = __tracing_attr_span.enter();
                    }
                    #[warn(clippy::suspicious_else_formatting)]
                    {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: Result<(), Error> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event commons/dimas-config/src/factory/xml_parser.rs:401",
                                            "dimas_config::factory::xml_parser",
                                            Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "commons/dimas-config/src/factory/xml_parser.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(401u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "dimas_config::factory::xml_parser",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!("parse_sub_xml") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let doc = Document::parse(xml)?;
                            let root = doc.root_element();
                            if root.tag_name().name() != "root" {
                                return Err(Error::RootName);
                            }
                            if let Some(format) = root.attribute("BTCPP_format") {
                                if format != "4" {
                                    return Err(Error::BtCppFormat);
                                }
                            }
                            if let Some(id) = root.attribute("main_tree_to_execute") {
                                return Err(Error::MainTreeNotAllowed);
                            }
                            Self::parse_document(root, data, blackboard)
                        }
                    }
                }
            }
        }
    }
    pub use error::Error;
    #[allow(clippy::module_name_repetitions)]
    pub use factory::BTFactory;
}
