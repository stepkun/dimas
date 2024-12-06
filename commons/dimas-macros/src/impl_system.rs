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
use crate::impl_operational;

type Arguments = Punctuated<Meta, Token![,]>;

const ARGS_UNSUPPORTED: &str = "arguments are not supported by 'system' macro";

#[derive(Debug, Default)]
struct Config {}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	if !args.is_empty() {
		return Err(syn::Error::new_spanned(args, ARGS_UNSUPPORTED));
	}

	Ok(config)
}

fn self_functions() -> TokenStream {
	quote! {
		fn system(&self) -> &SystemType {
			&self.system
		}

		fn system_mut(&mut self) -> &mut SystemType {
			&mut self.system
		}

		fn operational(&self) -> &OperationalType {
			self.system.operational()
		}

		fn operational_mut(&mut self) -> &mut OperationalType {
			self.system.operational_mut()
		}
	}
}

fn system_fields() -> TokenStream {
	quote! {
		system: SystemType,
	}
}

fn system_functions() -> TokenStream {
	quote! {
		#[inline]
		fn id(&self) -> String {
			self.system.id()
		}

		#[inline]
		fn set_id(&mut self, id: String){
			self.system.set_id(id);
		}

		#[inline]
		fn add_activity(&mut self, activity: Box<dyn Activity>) {
			self.system.add_activity(activity);
		}

		#[inline]
		fn remove_activity(&mut self, id: ActivityId) {
			self.system.remove_activity(id);
		}

		#[inline]
		fn activities(&self) -> parking_lot::RwLockReadGuard<Vec<Box<dyn Activity>>> {
			self.system.activities()
		}

		#[inline]
		fn activities_mut(&mut self) -> parking_lot::RwLockWriteGuard<Vec<Box<dyn Activity>>> {
			self.system.activities_mut()
		}

		#[inline]
		fn add_component(&mut self, component: Box<dyn Component>) {
			self.system.add_component(component);
		}

		#[inline]
		fn remove_component(&mut self, id: ComponentId) {
			self.system.remove_component(id);
		}

		#[inline]
		fn components(&self) -> parking_lot::RwLockReadGuard<Vec<Box<dyn Component>>> {
			self.system.components()
		}

		#[inline]
		fn components_mut(&mut self) -> parking_lot::RwLockWriteGuard<Vec<Box<dyn Component>>> {
			self.system.components_mut()
		}
	}
}

#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
fn system_struct(mut item: ItemStruct) -> Result<TokenStream> {
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
	let impl_header = create_impl_header(&item, None)?;
	let operational_header = create_impl_header(&item, Some("Operational"))?;
	let system_header = create_impl_header(&item, Some("System"))?;
	// create blocks
	let self_impl = self_functions();
	let operational_block = impl_operational::operational_functions();
	let system_block = system_functions();
	// create fields
	let system_fields = system_fields();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#struct_header {
			#system_fields
			#old_fields
		}

		// add the necessary impl for blocks after the struct
		#impl_header {
			#self_impl
		}

		#operational_header {
			#operational_block
		}

		#system_header {
			#system_block
		}
	};
	Ok(out)
}

pub fn system(args: TokenStream, input: TokenStream) -> TokenStream {
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
				Ok(args) => system_struct(item_struct).unwrap_or_else(Error::into_compile_error),
			}
		},
	)
}
