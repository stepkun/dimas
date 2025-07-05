// Copyright © 2025 Stephan Kunz

//! Derive macro [`ScriptEnum`] for `dimas-scripting`
//!

#[doc(hidden)]
extern crate proc_macro;

#[doc(hidden)]
extern crate alloc;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Lit};

/// Implementation of the derive macro [`ScriptEnum`]
fn derive_scripting_enum(input: &DeriveInput) -> TokenStream {
	// structure name
	let ident = &input.ident;

	// Check type of input and handle enums
	let mut discriminant = -1_i8;
	let variants: Vec<(String, i8)> = match &input.data {
		syn::Data::Enum(data) => data
			.variants
			.iter()
			.map(|v| {
				if let Some((_eq, expr)) = &v.discriminant {
					match expr {
						Expr::Lit(expr_lit) => match &expr_lit.lit {
							Lit::Int(lit_int) => {
								discriminant = lit_int
									.base10_parse::<i8>()
									.expect("value must be i8");
							}
							_ => panic!("value must be i8"),
						},
						_ => panic!("value must be i8"),
					}
				} else {
					discriminant += 1;
				}
				(v.ident.to_string(), discriminant)
			})
			.collect(),
		syn::Data::Struct(_struct) => panic!("structs not supported by ScriptEnum"),
		syn::Data::Union(_union) => panic!("unions not supported by ScriptEnum"),
	};
	let variant_keys: Vec<String> = variants.iter().map(|v| v.0.clone()).collect();
	let variant_discriminants: Vec<i8> = variants.iter().map(|v| v.1).collect();

	//
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let derived: TokenStream = "#[automatically_derived]"
		.parse()
		.expect("derive(ScriptEnum) - derived");
	let diagnostic: TokenStream = "#[diagnostic::do_not_recommend]"
		.parse()
		.expect("derive(ScriptEnum) - diagnostic");

	quote! {
		#derived
		#diagnostic
		impl #impl_generics dimas_scripting::ScriptEnum for #ident #type_generics #where_clause {
			fn key_value_tuples() -> alloc::vec::Vec<(&'static str, i8)> {
				vec![#((#variant_keys, #variant_discriminants)),*]
			}
		}
	}
}

/// Derive macro [`ScriptEnum`].
/// Enables a Rust enum to be used in a 'C' like mannner within the `DiMAS` scripting language.
///
/// # Usage
/// ```no_test
/// #[derive(ScriptEnum)]
/// enum MyEnum {
///     // specific elements
///     ...
/// }
///
/// impl MyEnum {
///     // specific implementations
///     ...
/// }
/// ```
///
/// # Result
/// Expands the above example to
/// ```no_test
/// enum MyEnum {
///     // specific elements
///     ...
/// }
///
/// impl MyEnum {
///     // specific implementations
///     ...
/// }
///
/// #[automatically_derived]
/// #[diagnostic::do_not_recommend]
/// impl dimas_scripting::enum::ScriptEnum for MyEnum {}
/// ```
///
/// # Errors
///
/// # Panics
/// - if used on structs or unions
#[proc_macro_derive(ScriptEnum, attributes(dimas))]
pub fn derive_script_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	// Construct a representation of Rust code as a syntax tree
	let input: DeriveInput = syn::parse(input).expect("could not parse input");

	derive_scripting_enum(&input).into()
}
