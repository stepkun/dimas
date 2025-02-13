// Copyright © 2024 Stephan Kunz

//! Macros for `DiMAS`
//! - `#[dimas::main(...)]`
//! - `#[dimas::agent]`
//!

#[doc(hidden)]
extern crate proc_macro;

mod behavior_impl;
mod behavior_struct;
mod impl_main;
mod register_behavior;
mod utils;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemImpl, ItemStruct, parse2};

/// Marks async main function to be executed by a multi threaded tokio runtime.
///
/// Note: This macro can only be used on the `main` function.
///
/// # Usage
/// ```no_test
/// #[dimas::main]
/// async fn main() {
///     // your code
///     ...
/// }
/// ```
///
/// ## Increase Worker threads
/// `DiMAS` creates a minimum of 3 worker threads within tokio runtime.
///
/// To increase the amount of worker threads, the macro can be configured using
///
/// ```no_test
/// #[dimas::main(additional_threads = 5)]  // adds additional 5 threads to the default of 3
/// ```
///
#[proc_macro_attribute]
pub fn main(metadata: TokenStream, input: TokenStream) -> TokenStream {
	// call implementation with conversion to and from proc-macro2 library
	impl_main::main(metadata.into(), input.into()).into()
}

/// Create a behavior from struct or impl block
/// @TODO: Documentation
#[proc_macro_attribute]
pub fn behavior(metadata: TokenStream, input: TokenStream) -> TokenStream {
	// check for impl or struct block
	if let Ok(item) = parse2::<ItemStruct>(input.clone().into()) {
		behavior_struct::entry(metadata.into(), item).into()
	} else if let Ok(item) = parse2::<ItemImpl>(input.into()) {
		behavior_impl::entry(metadata.into(), item).into()
	} else {
		syn::Error::new(
			Span::call_site(),
			"`behavior` macro must be used on `struct` or `impl` block",
		)
		.into_compile_error()
		.into()
	}
}

/// Register an Action
#[proc_macro]
pub fn register_action(input: TokenStream) -> TokenStream {
	register_behavior::register_behavior(
		input,
		quote! { ::dimas_core::behavior::BehaviorCategory::Action },
		register_behavior::BehaviorTypeInternal::Action,
	)
}

/// Register a Condition
#[proc_macro]
pub fn register_condition(input: TokenStream) -> TokenStream {
	register_behavior::register_behavior(
		input,
		quote! { ::dimas_core::behavior::BehaviorCategory::Condition },
		register_behavior::BehaviorTypeInternal::Condition,
	)
}

/// Register a Control
#[proc_macro]
pub fn register_control(input: TokenStream) -> TokenStream {
	register_behavior::register_behavior(
		input,
		quote! { ::dimas_core::behavior::BehaviorCategory::Control },
		register_behavior::BehaviorTypeInternal::Control,
	)
}

/// Register a Decorator
#[proc_macro]
pub fn register_decorator(input: TokenStream) -> TokenStream {
	register_behavior::register_behavior(
		input,
		quote! { ::dimas_core::behavior::BehaviorCategory::Decorator },
		register_behavior::BehaviorTypeInternal::Decorator,
	)
}

/// Register a function with signature `async fn function_name() -> BehaviorResult`.
/// A registered function can only be a behavior tree leaf - either an action or a condition.
///
/// The function must never return status "Running"
#[proc_macro]
pub fn register_function(input: TokenStream) -> TokenStream {
	register_behavior::register_function(input.into()).into()
}
