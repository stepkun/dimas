// Copyright © 2024 Stephan Kunz
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::match_same_arms)]

//! Macro `register_behavior` implementation
//!

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::token::Comma;
use syn::{punctuated::Punctuated, Token};

pub enum BehaviorTypeInternal {
	Action,
	Condition,
	Control,
	Decorator,
}

struct BehaviorRegistration {
	factory: syn::Ident,
	name: proc_macro2::TokenStream,
	bhvr_type: syn::Type,
	params: Punctuated<syn::Expr, Comma>,
}

impl Parse for BehaviorRegistration {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let factory = input.parse()?;
		input.parse::<Token![,]>()?;

		let bhvr_name = input.parse::<syn::Expr>()?.to_token_stream();

		input.parse::<Token![,]>()?;
		let bhvr_type = input.parse()?;
		// If there are extra parameters, try to parse a comma. Otherwise skip
		if !input.is_empty() {
			input.parse::<Token![,]>()?;
		}

		let params = input.parse_terminated(syn::Expr::parse, Token![,])?;

		Ok(Self {
			factory,
			name: bhvr_name,
			bhvr_type,
			params,
		})
	}
}

fn build_behavior(bhvr: &BehaviorRegistration) -> proc_macro2::TokenStream {
	let BehaviorRegistration {
		factory: _,
		name,
		bhvr_type,
		params,
	} = bhvr;

	let cloned_names = (0..params.len()).fold(quote! {}, |acc, i| {
		let arg_name = Ident::new(&format!("arg{i}"), Span::call_site());
		quote! { #acc, #arg_name.clone() }
	});

	quote! {
		{
			let mut bhvr = #bhvr_type::create_behavior(#name, config #cloned_names);
			let manifest = ::dimas_core::behavior::BehaviorManifest {
				bhvr_type: bhvr.bhvr_category(),
				registration_id: #name.into(),
				ports: bhvr.provided_ports(),
				description: ::alloc::string::String::new(),
			};
			bhvr.config_mut().set_manifest(::alloc::sync::Arc::new(manifest));
			bhvr
		}
	}
}

/// @TODO: ersetzen durch Lösung rein mit `proc_macro2`
pub fn register_behavior(
	input: TokenStream,
	bhvr_type_token: proc_macro2::TokenStream,
	bhvr_type: BehaviorTypeInternal,
) -> TokenStream {
	let bhvr_registration = parse_macro_input!(input as BehaviorRegistration);

	let factory = &bhvr_registration.factory;
	let name = &bhvr_registration.name;
	let params = &bhvr_registration.params;

	// Create expression with cloned parameter
	let param_clone_expr = params
		.iter()
		.enumerate()
		.fold(quote! {}, |acc, (i, item)| {
			let arg_name = Ident::new(&format!("arg{i}"), Span::call_site());
			quote! {
				#acc
				let #arg_name = #item.clone();
			}
		});

	let bhvr = build_behavior(&bhvr_registration);

	let extra_steps = match bhvr_type {
		BehaviorTypeInternal::Action | BehaviorTypeInternal::Condition => quote! {},
		BehaviorTypeInternal::Control | BehaviorTypeInternal::Decorator => quote! {
			bhvr.data.children = children;
		},
	};

	let expanded = quote! {
		{
			let blackboard = #factory.blackboard().clone();

			#param_clone_expr

			let bhvr_fn = move |
				config: ::dimas_core::behavior::BehaviorConfig,
				mut children: ::alloc::vec::Vec<::dimas_core::behavior::Behavior>
			| -> ::dimas_core::behavior::Behavior
			{
				let mut bhvr = #bhvr;

				#extra_steps

				bhvr
			};

			#factory.register_behavior(#name, bhvr_fn, #bhvr_type_token);
		}
	};

	TokenStream::from(expanded)
}
