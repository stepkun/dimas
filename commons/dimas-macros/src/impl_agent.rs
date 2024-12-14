// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]

//! Macro implementations
//!

extern crate alloc;

use alloc::string::String;

use proc_macro::LexError;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, TokenStreamExt};
use syn::{parse::Parser, punctuated::Punctuated, Fields, ItemFn, Meta, Result, Token};
use syn::{parse2, parse_macro_input, AttrStyle, Error, ItemStruct, Path};

use crate::utils::{
	collect_data, convert_attrs, convert_derives, create_agent_struct_header, create_agent_type,
	create_as_refs, create_impl_header, create_struct_header,
};

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'agent' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

/// Additional functions for the properties struct:
/// - `pub fn into_agent(self)`: a self consuming function to create an [`Agent`] with these properties
fn additional_item_functions(item: &ItemStruct) -> TokenStream {
	quote! {
		#[inline]
		pub fn into_agent(self) -> Agent {
			Agent::new(Box::new(self))
		}
	}
}

fn agent_struct(mut item: ItemStruct) -> Result<TokenStream> {
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

	// create necessary variables
	let item_impl_header = create_impl_header(&item, None)?;
	let additional_item_functions = additional_item_functions(&item);

	// create output stream
	let out = quote! {
		#item

		#item_impl_header {
			#additional_item_functions
		}
	};
	Ok(out)
}

pub fn agent(args: TokenStream, input: TokenStream) -> TokenStream {
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
				Ok(args) => agent_struct(item_struct).unwrap_or_else(Error::into_compile_error),
			}
		},
	)
}
