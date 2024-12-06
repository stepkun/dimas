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
	collect_data, convert_attrs, convert_derives, create_as_refs, create_impl_header,
	create_struct_header,
};

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'parameter' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

// fn self_functions() -> TokenStream {
// quote! {
// fn parameter(&self) -> &ParameterType {
// &self.parameter
// }
//
// fn parameter_mut(&mut self) -> &mut ParameterType {
// &mut self.parameter
// }
// }
// }
//
// pub fn parameter_fields() -> TokenStream {
// quote! {
// parameter: ParameterType,
// }
// }
//
// pub fn parameter_functions() -> TokenStream {
// quote! {
// #[inline]
// fn activation_state(&self) -> OperationState {
// self.parameter().activation_state()
// }
//
// #[inline]
// fn set_activation_state(&mut self, state: OperationState) {
// self.parameter_mut().set_activation_state(state);
// }
//
// #[inline]
// fn state(&self) -> OperationState {
// self.parameter().state()
// }
//
// #[inline]
// fn set_state(&mut self, state: OperationState) {
// self.parameter_mut().set_state(state);
// }
// }
// }

#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
fn parameter_struct(mut item: ItemStruct) -> Result<TokenStream> {
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
	// let impl_header = create_impl_header(&item, None)?;
	// let parameter_header = create_impl_header(&item, Some("Parameter"))?;
	// create blocks
	// let self_impl = self_functions();
	// let parameter_impl = parameter_functions();
	// create fields
	// let parameter_fields = parameter_fields();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#struct_header {
			#old_fields
		}
	};
	Ok(out)
}

pub fn parameter(args: TokenStream, input: TokenStream) -> TokenStream {
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
				Ok(args) => parameter_struct(item_struct).unwrap_or_else(Error::into_compile_error),
			}
		},
	)
}
