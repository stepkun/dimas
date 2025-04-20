// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Derive macros for `dimas-behavior`
//!

#[doc(hidden)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

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
/// impl BehaviorAllMethods for MyBehavior {}
///
/// impl BehaviorCreationMethods for Fallback {
///     fn create() -> Box<BehaviorCreationFn> {
///         Box::new(|| Box::new(Self::default()))
///     }
/// }
///
/// impl BehaviorTreeMethods for MyBehavior {}
///
/// impl BehaviorRedirectionMethods for MyBehavior {
///     fn static_provided_ports(&self) -> NewPortList {
///         Self::provided_ports()
///     }
/// }
/// ```
///
/// # Errors
///
/// # Panics
///
#[proc_macro_derive(Behavior)]
pub fn behavior_derive(input: TokenStream) -> TokenStream {
	// Construct a representation of Rust code as a syntax tree
	let ast: DeriveInput = syn::parse(input).expect("could not parse input");

	// structure name
	let ident = &ast.ident;
	// structure generics w/o where clause
	let generics = &ast.generics.params;
	let where_clause = &ast.generics.where_clause;

	let derived: proc_macro2::TokenStream = "#[automatically_derived]"
		.parse()
		.expect("derive(Behavior)");

	quote! {
		#derived
		impl<#generics> BehaviorAllMethods for #ident<#generics> #where_clause {}

		#derived
		impl<#generics> BehaviorCreationMethods for #ident<#generics> #where_clause {
			fn create() -> Box<BehaviorCreationFn> {
				Box::new(|| Box::new(Self::default()))
			}
		}

		#derived
		impl<#generics> BehaviorTreeMethods for #ident<#generics> #where_clause {}

		#derived
		impl<#generics> BehaviorRedirectionMethods for #ident<#generics> #where_clause {
			fn static_provided_ports(&self) -> NewPortList {
				Self::provided_ports()
			}
		}
	}
	.into()
}
