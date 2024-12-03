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

use crate::impl_operational;
use crate::utils::{
	collect_data, convert_attrs, convert_derives, create_impl_header, create_struct_header,
};

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'activity' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

pub fn activity_fields() -> TokenStream {
	quote! {
		id: String,
	}
}

pub fn activity_functions() -> TokenStream {
	quote! {
		#[inline]
		fn id(&self) -> String {
			self.id.clone()
		}

		#[inline]
		fn set_id(&mut self, id: String){
			self.id = id;
		}
	}
}

#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
fn activity_struct(mut item: ItemStruct) -> Result<TokenStream> {
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
	let activity_header = create_impl_header(&item, "Activity")?;
	// create blocks
	let operational_block = impl_operational::operational_functions();
	let activity_block = activity_functions();
	// create fields
	let operational_fields = impl_operational::operational_fields();
	let activity_fields = activity_fields();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#struct_header {
			#operational_fields
			#activity_fields
			#old_fields
		}

		// add the impl for block after the struct
		#operational_header {
			#operational_block
		}

		#activity_header {
			#activity_block
		}
	};
	Ok(out)
}

pub fn activity(args: TokenStream, input: TokenStream) -> TokenStream {
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
				Ok(args) => activity_struct(item_struct).unwrap_or_else(Error::into_compile_error),
			}
		},
	)
}
