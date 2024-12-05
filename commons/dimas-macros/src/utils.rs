// Copyright © 2024 Stephan Kunz

//! Utility functions for implementing macros
//!

use proc_macro2::{LexError, TokenStream};
use quote::quote;
use syn::{AttrStyle, Attribute, ItemStruct};

/// Collect existing attributes and derives
#[allow(clippy::explicit_iter_loop)]
#[allow(clippy::equatable_if_let)]
pub fn collect_data(
	item: &ItemStruct,
	derives: &mut Vec<TokenStream>,
	user_attrs: &mut Vec<Attribute>,
) -> Result<(), syn::Error> {
	for attr in item.attrs.iter() {
		if attr.path().is_ident("derive") {
			derives.push(attr.parse_args()?);
		} else if let AttrStyle::Outer = attr.style {
			user_attrs.push(attr.clone());
		}
	}
	Ok(())
}

/// Convert Vec of derive Paths into one [`TokenStream`]
pub fn convert_derives(derives: Vec<TokenStream>) -> TokenStream {
	derives
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
		})
}

// Convert user attribs into one TokenStream
#[allow(clippy::equatable_if_let)]
pub fn convert_attrs(user_attrs: Vec<Attribute>) -> TokenStream {
	user_attrs
		.into_iter()
		.fold(proc_macro2::TokenStream::new(), |acc, a| {
			// Only want to transfer outer attributes
			if let AttrStyle::Outer = a.style {
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
		})
}

pub fn create_struct_header(item: &ItemStruct) -> TokenStream {
	let generics = &item.generics.params;
	let where_clause = &item.generics.where_clause;
	let vis = &item.vis;
	let item_ident = &item.ident;
	quote! {
		#vis struct #item_ident <#generics> #where_clause
	}
}

pub fn create_impl_header(
	item: &ItemStruct,
	trait_name: Option<&str>,
) -> Result<TokenStream, LexError> {
	let generics = &item.generics.params;
	let where_clause = &item.generics.where_clause;
	let item_ident = &item.ident;
	if let Some(trait_name) = trait_name {
		let name = trait_name.parse::<TokenStream>()?;
		Ok(quote! {
			impl<#generics> #name for #item_ident<#generics> #where_clause
		})
	} else {
		Ok(quote! {
			impl<#generics> #item_ident<#generics> #where_clause
		})
	}
}

pub fn create_as_refs(item: &ItemStruct, trait_name: &str) -> Result<TokenStream, LexError> {
	let generics = &item.generics.params;
	let where_clause = &item.generics.where_clause;
	let name = trait_name.parse::<TokenStream>()?;
	let item_ident = &item.ident;
	let variable = trait_name.to_lowercase();
	let mut variable_mut = variable.clone();
	let variable = variable.parse::<TokenStream>()?;
	variable_mut.push_str("_mut");
	let variable_mut = variable_mut.parse::<TokenStream>()?;
	Ok(quote! {
		impl<#generics> AsRef<#name> for #item_ident<#generics> #where_clause {
			fn as_ref(&self) {
				self.#variable()
			}
		}

		impl<#generics> AsMut<#name> for #item_ident<#generics> #where_clause {
			fn as_mut(&mut self) {
				self.#variable_mut()
			}
		}
	})
}
