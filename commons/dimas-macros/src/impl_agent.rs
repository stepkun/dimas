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
/// - `pub fn agent()`: a function to create an agent with these properties
/// # Errors
/// - Parsing errors during macro expansion.
/// - Missing `Default` implementations for property struct. Can be derived!
fn additional_item_functions(item: &ItemStruct) -> Result<TokenStream> {
	let agent_type = create_agent_type(item)?;
	Ok(quote! {
		#[inline]
		pub fn agent() -> #agent_type {
			#agent_type::default()
		}
	})
}

fn agent_impl_block(item: &ItemStruct) -> Result<TokenStream> {
	let properties_ident = &item.ident;
	let agent_ident = create_agent_type(item)?;

	Ok(quote! {
		impl #agent_ident {
			#[inline]
			pub fn uuid(&self) -> Uuid {
				self.data.read().uuid.clone()
			}

			#[inline]
			pub fn name(&self) -> String {
				self.data.read().name.clone()
			}

			#[inline]
			pub fn set_name(self, name: &str) -> #agent_ident {
				self.data.write().name = name.into();
				self
			}

			#[inline]
			pub fn prefix(&self) -> String {
				self.data.read().prefix.clone()
			}

			#[inline]
			pub fn set_prefix(self, prefix: &str) -> #agent_ident {
				self.data.write().prefix = prefix.into();
				self
			}

			#[inline]
			pub fn read(&self) -> parking_lot::lock_api::RwLockReadGuard<'_, parking_lot::RawRwLock, #properties_ident> {
				self.properties.read()
			}

			#[inline]
			fn write(&self) -> parking_lot::lock_api::RwLockWriteGuard<'_, parking_lot::RawRwLock, #properties_ident> {
				self.properties.write()
			}

			#[inline]
			fn add_activity(&self, activity: Box<dyn Activity>) {
				self.structure.write().activities.push(activity);
			}

			#[inline]
			fn remove_activity(&self, _id: ActivityId) {
				todo!()
			}

			#[inline]
			fn add_component(&self, component: Box<dyn Component>) {
				self.structure.write().components.push(component);
			}

			#[inline]
			fn remove_component(&self, _id: ComponentId) {
				todo!()
			}
		}
	})
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
	let agent_struct_header = create_agent_struct_header(&item)?;
	let agent_impl_block = agent_impl_block(&item)?;
	let item_impl_header = create_impl_header(&item, None)?;
	let additional_item_functions = additional_item_functions(&item)?;
	let property_data = &item.ident;
	let agent_ident = create_agent_type(&item)?;

	// create output stream
	let out = quote! {
		#item

		#item_impl_header {
			#additional_item_functions
		}

		#[derive(Clone, Debug, Default)]
		#agent_struct_header {
			data: alloc::sync::Arc<parking_lot::RwLock<dimas_core::AgentData>>,
			structure: alloc::sync::Arc<parking_lot::RwLock<dimas_core::ComponentStruct>>,
			properties: alloc::sync::Arc<parking_lot::RwLock<#property_data>>,
		}

		#agent_impl_block
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
