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

fn behavior_fields(type_ident_str: &str) -> TokenStream {
	match type_ident_str {
		// asynchronous behaviors
		"Action" | "Condition" | "Control" | "Decorator" => {
			quote! {
				running_fn: Self::_on_running,
				start_fn: Self::_on_start,
			}
		}
		// others are synchronous
		_ => {
			quote! {
				running_fn: Self::_tick,
				start_fn: Self::_tick,
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
	let bhvr_specific_tokens = behavior_fields(&type_ident_str);

	// create output stream
	let out = quote! {
		#struct_user_attrs
		#[derive(#struct_derives)]
		#struct_vis struct #struct_ident <#struct_generics> #struct_where_clause #struct_fields

		impl<#struct_generics> #struct_ident <#struct_generics> #struct_where_clause {
			/// generated behavior creation function
			pub fn create_behavior(
				name: impl AsRef<str>,
				config: ::dimas_core::behavior::BehaviorConfig,
				#manual_fields_with_types
			) -> ::dimas_core::behavior::Behavior {
				let ctx = Self {
					#extra_fields
				};

				let bhvr_data = ::dimas_core::behavior::BehaviorData {
					name: name.as_ref().to_string(),
					type_str: ::alloc::string::String::from(#struct_name),
					bhvr_type: ::dimas_core::behavior::BehaviorType::#bhvr_type,
					bhvr_category: ::dimas_core::behavior::BehaviorCategory::#bhvr_category,
					config,
					status: ::dimas_core::behavior::BehaviorStatus::Idle,
					children: ::alloc::vec::Vec::new(),
					ports_fn: Self::_ports,
				};

				::dimas_core::behavior::Behavior {
					data: bhvr_data,
					context: ::alloc::boxed::Box::new(ctx),
					#bhvr_specific_tokens
					halt_fn: Self::_halt,
				}
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
