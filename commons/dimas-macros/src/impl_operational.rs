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

use crate::utils::{
	collect_data, convert_attrs, convert_derives, create_impl_header, create_struct_header,
};

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

#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
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

	// collect existing data
	collect_data(&item, &mut derives, &mut user_attrs);
	// Convert Vec of derive Paths into one TokenStream
	let derives = convert_derives(derives);
	// Convert Vec of user attribs into one TokenStream
	let user_attrs = convert_attrs(user_attrs);

	// create headers
	let struct_header = create_struct_header(&item);
	let operational_header = create_impl_header(&item, "Operational")?;
	// create blocks
	let operational_block = operational_functions();
	// create fields
	let operational_fields = operational_fields();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#struct_header {
			#operational_fields
			#old_fields
		}

		// add the impl for block after the struct
		#operational_header {
			#operational_block
		}
	};
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
