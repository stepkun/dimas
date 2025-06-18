// Copyright © 2025 Stephan Kunz

//! Derive macro [`Behavior`] for `dimas-behavior`
//!

#[doc(hidden)]
extern crate proc_macro;

#[doc(hidden)]
extern crate alloc;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Implementation of the derive macro [`Behavior`]
fn derive_behavior_struct(input: &DeriveInput) -> TokenStream {
	// structure name
	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let derived: TokenStream = "#[automatically_derived]"
		.parse()
		.expect("derive(Behavior) - derived");
	let diagnostic: TokenStream = "#[diagnostic::do_not_recommend]"
		.parse()
		.expect("derive(Behavior) - diagnostic");

	quote! {
		#derived
		#diagnostic
		impl #impl_generics dimas_behavior::behavior::Behavior for #ident #type_generics #where_clause {}

		#derived
		#diagnostic
		impl #impl_generics dimas_behavior::behavior::BehaviorCreation for #ident #type_generics #where_clause {
			fn creation_fn() -> alloc::boxed::Box<dimas_behavior::behavior::BehaviorCreationFn> {
				alloc::boxed::Box::new(|| alloc::boxed::Box::new(Self::default()))
			}
		}

		#derived
		#diagnostic
		impl #impl_generics dimas_behavior::behavior::BehaviorExecution for #ident #type_generics #where_clause {
			fn as_any(&self) -> &dyn core::any::Any { self }
			fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
		}

		#derived
		#diagnostic
		impl #impl_generics dimas_behavior::behavior::BehaviorRedirection for #ident #type_generics #where_clause {
			fn static_provided_ports(&self) -> dimas_behavior::port::PortList {
				Self::provided_ports()
			}
		}
	}
}

/// Derive macro for [`Behavior`].
///
/// # Usage
/// ```no_test
/// #[derive(Behavior)]
/// struct MyBehavior {
///     // specific elements
///     ...
/// }
///
/// impl MyBehavior {
///     // specific implementations
///     ...
/// }
/// ```
///
/// # Result
/// Expands the above example to
/// ```no_test
/// struct MyBehavior {
///     // specific elements
///     ...
/// }
///
/// impl MyBehavior {
///     // specific implementations
///     ...
/// }
///
/// #[automatically_derived]
/// #[diagnostic::do_not_recommend]
/// impl dimas_behavior::behavior::Behavior for MyBehavior {}
///
/// #[automatically_derived]
/// #[diagnostic::do_not_recommend]
/// impl dimas_behavior::behavior::BehaviorCreation for Fallback {
///     fn creation_fn() -> alloc::boxed::Box<dimas_behavior::behavior::BehaviorCreationFn> {
///         alloc::boxed::Box::new(|| alloc::boxed::Box::new(Self::default()))
///     }
/// }
///
/// #[automatically_derived]
/// #[diagnostic::do_not_recommend]
/// impl dimas_behavior::behavior::BehaviorExecution for MyBehavior {
///     fn as_any(&self) -> &dyn core::any::AnyAny { self }
///     fn as_any_mut(&mut self) -> &mut dyn core::any::AnyAny { self }
/// }
///
/// #[automatically_derived]
/// #[diagnostic::do_not_recommend]
/// impl dimas_behavior::behavior::BehaviorRedirection for MyBehavior {
///     fn static_provided_ports(&self) -> dimas_behavior::port::PortList {
///         Self::provided_ports()
///     }
/// }
/// ```
///
/// # Errors
///
/// # Panics
/// - if used on enums or unions
#[proc_macro_derive(Behavior, attributes(dimas))]
pub fn derive_behavior(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	// Construct a representation of Rust code as a syntax tree
	let input: DeriveInput = syn::parse2(input.into()).expect("could not parse input");

	// Check type of input
	match &input.data {
		syn::Data::Struct(_struct) => derive_behavior_struct(&input).into(),
		syn::Data::Enum(_enum) => panic!("enums not supported"),
		syn::Data::Union(_union) => panic!("unions not supported"),
	}
}
