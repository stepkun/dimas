// Copyright © 2024 Stephan Kunz

//! Macro `behavior-impl` implementation
//!

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::visit_mut::{self, VisitMut};
use syn::{Error, FnArg, GenericParam, ImplItem, ImplItemFn, ItemImpl, ReturnType, parse2};
use syn::{Meta, Result, Token, parse::Parser, punctuated::Punctuated};

type Arguments = Punctuated<Meta, Token![,]>;

const UNSUPPORTED: &str = "not supported by macro";
const MISSING_BEHAVIOR_TYPE: &str = "missing behavior type";

#[derive(Debug)]
struct Config {
	bhvr_type: Ident,
	start_fn: Option<Ident>,
	running_fn: Ident,
	halt_fn: Option<Ident>,
	port_fn: Option<Ident>,
}

fn parse_config(args: Arguments) -> Result<Config> {
	if args.is_empty() {
		return Err(Error::new_spanned(args, MISSING_BEHAVIOR_TYPE));
	}

	let mut bhvr_type = format_ident!("SyncAction");
	let mut start_fn = None;
	let mut tick_fn = format_ident!("tick");
	let halt_fn = None;
	let port_fn = None;

	for arg in args {
		match arg {
			Meta::List(list) => {
				return Err(syn::Error::new_spanned(&list, UNSUPPORTED));
			}
			Meta::NameValue(named_value) => {
				return Err(syn::Error::new_spanned(&named_value, UNSUPPORTED));
			}
			Meta::Path(path) => {
				let ident = path
					.get_ident()
					.ok_or_else(|| syn::Error::new_spanned(&path, "must have a specified ident"))?;
				ident.clone_into(&mut bhvr_type);

				let bhvr_type_str = bhvr_type.to_string();
				if !bhvr_type_str.starts_with("Sync") {
					tick_fn = Ident::new("on_running", Span::call_site());
					start_fn = Some(Ident::new("on_start", Span::call_site()));
				}
			}
		}
	}

	Ok(Config {
		bhvr_type,
		start_fn,
		running_fn: tick_fn,
		halt_fn,
		port_fn,
	})
}

struct SelfVisitor;

impl VisitMut for SelfVisitor {
	fn visit_ident_mut(&mut self, i: &mut proc_macro2::Ident) {
		if i == "self" {
			let ctx = quote! { self_ };
			let ctx = parse2(ctx).unwrap_or_else(|_| todo!());

			*i = ctx;
		}

		visit_mut::visit_ident_mut(self, i);
	}
}

fn alter_behavior_fn(fn_item: &mut ImplItemFn, is_async: bool) -> Result<()> {
	// Remove async
	if is_async {
		fn_item.sig.asyncness = None;
	}
	// Add lifetime to signature
	let lifetime: GenericParam = parse2(quote! { 'a })?;
	fn_item.sig.generics.params.push(lifetime);
	// Rename parameters
	for arg in &mut fn_item.sig.inputs {
		if let FnArg::Receiver(_) = arg {
			let new_arg = quote! { bhvr_: &'a mut ::dimas_core::behavior::BehaviorData };
			let new_arg = parse2(new_arg)?;
			*arg = new_arg;
		}
	}

	let new_arg = parse2(
		quote! { ctx: &'a mut ::alloc::boxed::Box<dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync> },
	)?;

	fn_item.sig.inputs.push(new_arg);

	let old_block = &mut fn_item.block;

	// Rename occurrences of self
	SelfVisitor.visit_block_mut(old_block);

	let new_block = if is_async {
		// Get old return without the -> token
		let old_return = match &fn_item.sig.output {
			ReturnType::Default => quote! { () },
			ReturnType::Type(_, ret) => quote! { #ret },
		};

		// Wrap return type in BoxFuture
		let new_return = quote! {
			-> ::futures::future::BoxFuture<'a, #old_return>
		};

		let new_return = parse2(new_return)?;
		fn_item.sig.output = new_return;

		// Wrap function block in Box::pin and create ctx
		quote! {
			{
				::alloc::boxed::Box::pin(async move {
					let mut self_ = ctx.downcast_mut::<Self>().unwrap();
					#old_block
				})
			}
		}
	} else {
		// create ctx
		quote! {
			{
				let mut self_ = ctx.downcast_mut::<Self>().unwrap();
				#old_block
			}
		}
	};

	let new_block = parse2(new_block)?;

	fn_item.block = new_block;

	Ok(())
}

#[allow(clippy::explicit_iter_loop)]
fn behavior_impl(mut args: Config, mut item: ItemImpl) -> Result<TokenStream> {
	for sub_item in item.items.iter_mut() {
		if let ImplItem::Fn(fn_item) = sub_item {
			let mut should_rewrite_def = false;
			// Rename methods
			let mut new_ident = None;
			// Check if it's a tick
			if fn_item.sig.ident == args.running_fn {
				new_ident = if args.bhvr_type == "Action"
					|| args.bhvr_type == "Condition"
					|| args.bhvr_type == "Control"
					|| args.bhvr_type == "Decorator"
				{
					// asynchronous behaviors
					Some(parse2(quote! { _on_running })?)
				} else {
					// synchronous behaviors
					Some(parse2(quote! { _tick })?)
				};

				should_rewrite_def = true;
			}
			// Check if it's an on_start
			if let Some(on_start) = args.start_fn.as_ref() {
				if &fn_item.sig.ident == on_start {
					new_ident = Some(parse2(quote! { _on_start })?);
					should_rewrite_def = true;
				}
			}
			// Check if it's a halt
			if let Some(on_halt) = args.halt_fn.as_ref() {
				if &fn_item.sig.ident == on_halt {
					new_ident = Some(parse2(quote! { _halt })?);
					should_rewrite_def = true;
				}
			} else if &fn_item.sig.ident == "halt" {
				args.halt_fn = Some(fn_item.sig.ident.clone());
				new_ident = Some(parse2(quote! { _halt })?);
				should_rewrite_def = true;
			}
			// Check if it's a ports
			if let Some(port_fn) = args.port_fn.as_ref() {
				if &fn_item.sig.ident == port_fn {
					new_ident = Some(parse2(quote! { _ports })?);
				}
			} else if &fn_item.sig.ident == "ports" {
				args.port_fn = Some(fn_item.sig.ident.clone());
				new_ident = Some(parse2(quote! { _ports })?);
			}

			if let Some(new_ident) = new_ident {
				if should_rewrite_def {
					alter_behavior_fn(fn_item, true)?;
				}

				fn_item.sig.ident = new_ident;
			}
		}
	}

	let mut extra_impls = Vec::new();

	if args.halt_fn.is_none() {
		extra_impls.push(parse2(quote! {
            fn _halt<'a>(bhvr_: &'a mut ::dimas_core::behavior::BehaviorData, ctx: &'a mut ::alloc::boxed::Box<dyn ::core::any::Any + ::core::marker::Send + ::core::marker::Sync>) -> ::futures::future::BoxFuture<'a, ()> { ::alloc::boxed::Box::pin(async move {}) }
        })?);
	}

	if args.port_fn.is_none() {
		extra_impls.push(parse2(quote! {
			fn _ports() -> ::dimas_core::port::PortList { ::dimas_core::port::PortList::new() }
		})?);
	}

	item.items.extend(extra_impls);

	Ok(quote! { #item })
}

pub fn entry(metadata: TokenStream, item: ItemImpl) -> TokenStream {
	// parse args
	let args = Arguments::parse_terminated
		.parse2(metadata)
		.and_then(parse_config);
	match args {
		Err(err) => err.into_compile_error(),
		Ok(config) => behavior_impl(config, item).unwrap_or_else(Error::into_compile_error),
	}
}
