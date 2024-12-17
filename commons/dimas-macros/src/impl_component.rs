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
	collect_data, common_derives, convert_attrs, convert_derives, create_impl_header,
	create_struct_header,
};

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'component' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

fn component_functions() -> TokenStream {
	quote! {
		#[inline]
		fn uuid(&self) -> Uuid {
			self.data.uuid.clone()
		}

		#[inline]
		fn id(&self) -> ComponentId {
			self.data.id.clone()
		}

		#[inline]
		fn version(&self) -> u32 {
			self.data.version
		}

		#[inline]
		fn add_activity(&mut self, activity: Box<dyn Activity>) {
			self.data.activities.push(activity);
		}

		#[inline]
		fn remove_activity(&mut self, _id: ActivityId) {
			todo!()
		}

		#[inline]
		fn add_component(&mut self, component: Box<dyn Component>) {
			self.data.components.push(component);
		}

		#[inline]
		fn remove_component(&mut self, _id: ComponentId) {
			todo!()
		}
	}
}

#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
fn component_struct(mut item: ItemStruct) -> Result<TokenStream> {
	// check for struct with named fields
	let original_fields = match &item.fields {
		Fields::Named(fields) => fields.named.clone(),
		_ => {
			return Err(syn::Error::new_spanned(
				item,
				"expecting a struct with named fields",
			))
		}
	};

	// collect existing data
	let mut derives = common_derives();
	let mut user_attrs = Vec::new();
	collect_data(&item, &mut derives, &mut user_attrs);
	// Convert Vec of derive Paths into one TokenStream
	let derives = convert_derives(derives);
	// Convert Vec of user attribs into one TokenStream
	let user_attrs = convert_attrs(user_attrs);

	// create necessary variables
	let item_struct_header = create_struct_header(&item);
	let component_impl_header = create_impl_header(&item, Some("Component"))?;
	let component_functions = component_functions();

	// create output stream
	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#item_struct_header {
			data: dimas_core::ComponentData,
			#original_fields
		}

		#component_impl_header {
			#component_functions
		}
	};
	Ok(out)
}

pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
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
				Ok(args) => component_struct(item_struct).unwrap_or_else(Error::into_compile_error),
			}
		},
	)
}
