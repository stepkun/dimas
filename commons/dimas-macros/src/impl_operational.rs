// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::needless_pass_by_value)]

//! Macro implementations
//!

extern crate alloc;

use alloc::string::String;

use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, TokenStreamExt};
use syn::{parse::Parser, punctuated::Punctuated, Fields, ItemFn, Meta, Result, Token};
use syn::{parse2, parse_macro_input, AttrStyle, Error, ItemStruct, Path};

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'operational' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

pub fn operational_fields() -> TokenStream {
	quote! {
		operational: OperationalType,
	}
}

pub fn operational_functions() -> TokenStream {
	quote! {
		#[inline]
		fn activation_state(&self) -> OperationState {
			self.operational.activation_state()
		}

		#[inline]
		fn set_activation_state(&mut self, state: OperationState) {
			self.operational.set_activation_state(state);
		}

		#[inline]
		fn state(&self) -> OperationState {
			self.operational.state()
		}

		#[inline]
		fn set_state(&mut self, state: OperationState) {
			self.operational.set_state(state);
		}
	}
}

fn operational_struct(mut item: ItemStruct) -> Result<TokenStream> {
	// check for struct with named fields
	let old_fields = match &item.fields {
		Fields::Named(fields) => fields.named.clone(),
		_ => {
			return Err(syn::Error::new_spanned(
				item,
				"expecting a struct with named fields",
			))
		}
	};

	// prepare variables
	let mut derives = super::common_derives();
	let mut user_attrs = Vec::new();

	let vis = &item.vis;
	let item_ident = &item.ident;

	// collect existing data
	for attr in item.attrs {
		if attr.path().is_ident("derive") {
			derives.push(attr.parse_args()?);
		} else if attr.style == AttrStyle::Outer {
			user_attrs.push(attr);
		}
	}

	let operational_block = operational_functions();

	// Convert Vec of derive Paths into one TokenStream
	let derives = derives
		.into_iter()
		.fold(proc_macro2::TokenStream::new(), |acc, d| {
			if acc.is_empty() {
				quote! {
					#d
				}
			} else {
				quote! {
					#acc, #d
				}
			}
		});

	//
	let user_attrs = user_attrs
		.into_iter()
		.fold(proc_macro2::TokenStream::new(), |acc, a| {
			// Only want to transfer outer attributes
			if a.style == AttrStyle::Outer {
				if acc.is_empty() {
					quote! {
						#a
					}
				} else {
					quote! {
						#acc
						#a
					}
				}
			} else {
				acc
			}
		});

	let new_fields = operational_fields();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#vis struct #item_ident {
			#new_fields
			#old_fields
		}

		// add the impl for block after the struct
		impl Operational for #item_ident {
			#operational_block
		}
	};
	//dbg!(out.to_string());
	Ok(out)
}

pub fn operational(args: TokenStream, input: TokenStream) -> TokenStream {
	parse2::<ItemStruct>(input).map_or_else(
		|_| {
			Error::new(Span::call_site(), "macro must be used on a `struct` block.")
				.into_compile_error()
		},
		|item_struct| {
			// parse args
			let args = Arguments::parse_terminated
				.parse2(args)
				.and_then(parse_config);
			match args {
				Err(err) => err.into_compile_error(),
				Ok(args) => {
					operational_struct(item_struct).unwrap_or_else(Error::into_compile_error)
				}
			}
		},
	)
}
