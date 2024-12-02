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

use crate::{impl_activity, impl_operational};

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

fn component_fields() -> TokenStream {
	quote! {
		id: ComponentId,
		activities: Arc<RwLock<Vec<Box<dyn Activity>>>>,
		components: Arc<RwLock<Vec<Box<dyn Component>>>>,
	}
}

fn component_functions() -> TokenStream {
	quote! {
		#[inline]
		fn id(&self) -> String {
			self.id.clone()
		}

		#[inline]
		fn set_id(&mut self, id: String){
			self.id = id;
		}

		#[inline]
		fn add(&mut self, component: Box<dyn Component>) {
			self.components.write().push(component);
		}

		#[inline]
		fn remove(&mut self, _id: ComponentId) {
			todo!()
		}

		#[inline]
		fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>> {
			self.activities.read()
		}

		#[inline]
		fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>> {
			self.activities.write()
		}

		#[inline]
		fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
			self.components.read()
		}

		#[inline]
		fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
			self.components.write()
		}
	}
}

fn component_struct(mut item: ItemStruct) -> Result<TokenStream> {
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

	let operational_fields = impl_operational::operational_fields();
	let component_fields = component_fields();
	let operational_functions = impl_operational::operational_functions();
	let component_functions = component_functions();

	let out = quote! {
		#user_attrs
		#[derive(#derives)]
		#vis struct #item_ident {
			#operational_fields
			#component_fields
			#old_fields
		}

		// add the impl for block after the struct
		impl Operational for #item_ident {
			#operational_functions
		}

		impl Component for #item_ident {
			#component_functions
		}
	};
	//dbg!(out.to_string());
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
