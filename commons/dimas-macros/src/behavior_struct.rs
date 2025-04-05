// Copyright © 2024 Stephan Kunz

//! Macro `behavior` implementation
//!

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::token::Comma;
use syn::{Error, ItemStruct, LitStr};
use syn::{Fields, Meta, Result, parse::Parser, punctuated::Punctuated};

use crate::utils::{
	ArgListToMap, Arguments, ConcatTokenStream, collect_data, determine_type_category,
};

const UNSUPPORTED: &str = "not supported by macro";
const MISSING_BEHAVIOR_TYPE: &str = "missing behavior type";

#[derive(Debug)]
struct Config {
	bhvr_type: Ident,
}

impl Default for Config {
	fn default() -> Self {
		let bhvr_type = format_ident!("SyncControl");
		Self { bhvr_type }
	}
}

fn parse_config(args: Arguments) -> Result<Config> {
	let mut config = Config::default();

	//dbg!(&args);

	if args.is_empty() {
		return Err(Error::new_spanned(args, MISSING_BEHAVIOR_TYPE));
	}

	for arg in args {
		match arg {
			Meta::List(list) => {
				return Err(Error::new_spanned(&list, UNSUPPORTED));
			}
			Meta::NameValue(named_value) => {
				return Err(Error::new_spanned(&named_value, UNSUPPORTED));
			}
			Meta::Path(path) => {
				let ident = path
					.get_ident()
					.ok_or_else(|| Error::new_spanned(&path, "must have a specified ident"))?;

				ident.clone_into(&mut config.bhvr_type);
			}
		}
	}

	Ok(config)
}

fn tick_functions(type_ident_str: &str) -> TokenStream {
	match type_ident_str {
		// asynchronous behaviors
		"Action" | "Condition" | "Control" | "Decorator" => {
			quote! {
				Self::_on_start,
				Self::_on_running,
			}
		}
		// others are synchronous
		_ => {
			quote! {
				Self::_tick,
				Self::_tick,
			}
		}
	}
}

fn behavior_struct(config: &Config, mut item: ItemStruct) -> Result<TokenStream> {
	// structure name
	let struct_ident = &item.ident;
	// structure visibility
	let struct_vis = &item.vis;
	// structure generics w/o where clause
	let struct_generics = &item.generics.params;
	let struct_where_clause = &item.generics.where_clause;
	// collect existing derives & macros
	let (struct_user_attrs, struct_derives) = collect_data(&item)?;

	// check for struct with named fields and collect fields
	let mut default_fields = TokenStream::new();
	let mut manual_fields = TokenStream::new();
	let mut manual_fields_with_types = TokenStream::new();
	match &mut item.fields {
		Fields::Named(fields) => {
			for f in &mut fields.named {
				let name = f.ident.as_ref().unwrap_or_else(|| todo!());
				let ty = &f.ty;

				let mut default_given = false;
				for a in &f.attrs {
					if a.path().is_ident("bhvr") {
						let args: Punctuated<syn::Meta, Comma> =
							a.parse_args_with(Punctuated::parse_terminated)?;
						let args_map = args.arglist_to_map()?;

						// `default` argument given?
						if let Some(value) = args_map.get(&syn::parse_str("default")?) {
							default_given = true;
							// user provided a distinct default?
							let default_value = value.as_ref().map_or_else(
								|| quote! { <#ty>::default() },
								|default_value| quote! { #default_value },
							);

							default_fields =
								default_fields.concat_list(quote! { #name: #default_value });
						}
					}
				}

				// field has to be manually defined
				if !default_given {
					manual_fields = manual_fields.concat_list(quote! { #name });
					manual_fields_with_types =
						manual_fields_with_types.concat_list(quote! { #name: #ty });
				}

				// remove only the 'bhvr' attribute
				f.attrs = f
					.attrs
					.clone()
					.into_iter()
					.filter(|a| !a.path().is_ident("bhvr"))
					.collect();
			}
		}
		_ => {
			return Err(Error::new_spanned(
				item,
				"expecting a struct with named fields",
			));
		}
	}

	let extra_fields = TokenStream::new()
		.concat_list(default_fields)
		.concat_list(manual_fields);

	let struct_name = LitStr::new(
		&struct_ident.to_token_stream().to_string(),
		struct_ident.span(),
	);
	let struct_fields = &item.fields;

	let type_ident_str = config.bhvr_type.to_string();
	let (bhvr_type, bhvr_category) = determine_type_category(&type_ident_str)?;
	let tick_functions = tick_functions(&type_ident_str);

	// create output stream
	let out = quote! {
		#struct_user_attrs
		#[derive(#struct_derives)]
		#struct_vis struct #struct_ident <#struct_generics> #struct_where_clause #struct_fields

		impl<#struct_generics> #struct_ident <#struct_generics> #struct_where_clause {
			/// generated behavior creation function
			pub fn create_behavior(
				name: impl AsRef<str>,
				config: ::dimas_behavior::behavior::BehaviorConfig,
				#manual_fields_with_types
			) -> ::dimas_behavior::behavior::Behavior {
				let ctx = Self {
					#extra_fields
				};

				let bhvr_data = ::dimas_behavior::behavior::BehaviorData::new(
					name.as_ref().to_string(),
					::alloc::string::String::from(#struct_name),
					::dimas_behavior::behavior::BehaviorType::#bhvr_type,
					::dimas_behavior::behavior::BehaviorCategory::#bhvr_category,
					config,
					::dimas_behavior::behavior::BehaviorStatus::Idle,
					::alloc::vec::Vec::default(),
					Self::_ports,
				);

				::dimas_behavior::behavior::Behavior::new(
					bhvr_data,
					::alloc::boxed::Box::new(ctx),
					#tick_functions
					Self::_halt,
				)
			}
		}

	};
	Ok(out)
}

pub fn entry(metadata: TokenStream, item: ItemStruct) -> TokenStream {
	// parse args
	let args = Arguments::parse_terminated
		.parse2(metadata)
		.and_then(parse_config);
	match args {
		Err(err) => err.into_compile_error(),
		Ok(config) => behavior_struct(&config, item).unwrap_or_else(Error::into_compile_error),
	}
}
