// Copyright © 2024 Stephan Kunz

//! Utility functions for implementing macros

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	AttrStyle, Attribute, Error, Expr, ExprCall, Ident, ItemStruct, Lit, Meta, Path, Result, Token,
	parse2, punctuated::Punctuated, token::Comma,
};

/// helper type for handling of argument lists
pub type Arguments = Punctuated<Meta, Token![,]>;

/// Determine [`Behavior`s] type and category.
/// First field is [`BehaviorType`], second [`BehaviorCategory`]
pub fn determine_type_category(ident: &str) -> Result<(Path, Path)> {
	match ident {
		"Action" => Ok((
			parse2::<Path>(quote! { Action })?,
			parse2::<Path>(quote! { Action })?,
		)),
		"Condition" => Ok((
			parse2::<Path>(quote! { Condition })?,
			parse2::<Path>(quote! { Condition })?,
		)),
		"Control" => Ok((
			parse2::<Path>(quote! { Control })?,
			parse2::<Path>(quote! { Control })?,
		)),
		"Decorator" => Ok((
			parse2::<Path>(quote! { Decorator })?,
			parse2::<Path>(quote! { Decorator })?,
		)),
		"SyncAction" => Ok((
			parse2::<Path>(quote! { SyncAction })?,
			parse2::<Path>(quote! { Action })?,
		)),
		"SyncCondition" => Ok((
			parse2::<Path>(quote! { SyncCondition })?,
			parse2::<Path>(quote! { Condition })?,
		)),
		"SyncControl" => Ok((
			parse2::<Path>(quote! { SyncControl })?,
			parse2::<Path>(quote! { Condition })?,
		)),
		"SyncDecorator" => Ok((
			parse2::<Path>(quote! { SyncDecorator })?,
			parse2::<Path>(quote! { Condition })?,
		)),
		_ => Err(Error::new_spanned(ident, "invalid behavior type")),
	}
}

/// Collect existing attribute macros and derive macros
pub fn collect_data(item: &ItemStruct) -> Result<(TokenStream, TokenStream)> {
	let mut derives: Vec<TokenStream> = Vec::new();
	let mut user_attrs: Vec<Attribute> = Vec::new();

	for attr in &item.attrs {
		if attr.path().is_ident("derive") {
			derives.push(attr.parse_args()?);
		} else if attr.style == AttrStyle::Outer {
			user_attrs.push(attr.clone());
		}
	}

	// Convert Vec of user attribs into one TokenStream
	let user_attrs = convert_attrs(user_attrs);
	// Convert Vec of derive Paths into one TokenStream
	let derives = convert_derives(derives);

	Ok((user_attrs, derives))
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
pub fn convert_attrs(user_attrs: Vec<Attribute>) -> TokenStream {
	user_attrs
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
		})
}

/// helper trait to allow implementation for foreign type
pub trait ArgListToMap<T, K, V> {
	fn arglist_to_map(&self) -> Result<HashMap<K, V>>;
}

impl ArgListToMap<Self, Ident, Option<TokenStream>> for Punctuated<Meta, Comma> {
	/// Convert a list of attribute arguments to a `HashMap`
	fn arglist_to_map(&self) -> Result<HashMap<Ident, Option<TokenStream>>> {
		self.iter()
			.map(|m| {
				match m {
					Meta::NameValue(arg) => {
						// convert `Expr` to one of the valid types:
						if let Expr::Lit(lit) = &arg.value {
							// extract the literal
							if let Lit::Str(arg_str) = &lit.lit {
								let value = {
									// function call
									if let Ok(call) = arg_str.parse::<ExprCall>() {
										quote! { #call }
									}
									// variable name
									else if let Ok(ident) = arg_str.parse::<Ident>() {
										quote! { #ident }
									}
									// literal for integer types etc.
									else if let Ok(lit) = arg_str.parse::<Lit>() {
										quote! { #lit }
									}
									// variable type
									else if let Ok(path) = arg_str.parse::<Path>() {
										quote! { #path }
									} else {
										return Err(Error::new_spanned(
											&arg.value,
											"invalid argument, should be: variable, literal, path, function call",
										));
									}
								};
								let v = arg.path.get_ident().unwrap_or_else(|| todo!());
								Ok((v.clone(), Some(value)))
							} else {
								Err(Error::new_spanned(
									&arg.value,
									"value should be a string literal",
								))
							}
						} else {
							Err(Error::new_spanned(
								&arg.value,
								"value should be a string literal",
							))
						}
					}
					Meta::Path(arg) => {
						let v = arg.get_ident().unwrap_or_else(|| todo!());
						Ok((v.clone(), None))
					}
					Meta::List(_) => Err(Error::new_spanned(
						m,
						"attribute should be `#[bhvr(default)]` or `#[bhvr(default = \"String::new()\")]`",
					)),
				}
			})
			.collect()
	}
}

pub trait ConcatTokenStream {
	fn concat_list(&self, value: TokenStream) -> Self;
}

impl ConcatTokenStream for TokenStream {
	fn concat_list(&self, value: TokenStream) -> Self {
		if self.is_empty() {
			if value.is_empty() {
				// Both are empty
				Self::new()
			} else {
				// self empty, value not empty
				quote! {
					#value
				}
			}
		} else if value.is_empty() {
			// self not empty, value empty
			quote! {
				#self
			}
		} else {
			// Both have value
			quote! {
				#self,
				#value
			}
		}
	}
}
