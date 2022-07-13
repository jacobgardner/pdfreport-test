extern crate proc_macro;

mod config;
mod field_options;
mod output;

use config::{MERGEABLE_NAME, UNMERGEABLE_NAME};
use darling::FromMeta;
use field_options::{extract_field_attrs, FieldOptions, FieldsOptions};
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use syn::{
    self, braced,
    parse::Parse,
    parse_macro_input, parse_quote,
    token::{Brace, Struct},
    Attribute, AttributeArgs, Data, DataStruct, DeriveInput, ExprStruct, Fields, Ident, ItemStruct,
    LitStr, Token, Type,
};

fn build_skip_optional_attr() -> Attribute {
    parse_quote! { #[serde(skip_serializing_if = "Option::is_none")] }
}

fn convert_fields_to_optional(
    ast: &mut DeriveInput,
    fields_options: &FieldsOptions,
    global_options: &FieldOptions,
) {
    if let Data::Struct(mergeable_struct) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut mergeable_struct.fields {
            named_fields.named.iter_mut().for_each(|field| {
                let field_options = fields_options.get_by_field(field);

                let mergeable_type = if field_options.is_nested {
                    let ty_name = format!("{}::{}", field.ty.to_token_stream(), MERGEABLE_NAME);
                    Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
                } else {
                    field.ty.clone()
                };

                field.vis = parse_quote! { pub };
                if !global_options.use_null_in_serde {
                    field.attrs.push(build_skip_optional_attr());
                }
                field.ty = parse_quote! { Option< #mergeable_type > };
            });
        }
    } else {
        unimplemented!()
    }
}

fn convert_nested_fields(
    ast: &mut DeriveInput,
    fields_options: &FieldsOptions,
    _global_options: &FieldOptions,
) {
    if let Data::Struct(mergeable_struct) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut mergeable_struct.fields {
            named_fields.named.iter_mut().for_each(|field| {
                let field_options = fields_options.get_by_field(field);

                let mergeable_type = if field_options.is_nested {
                    let ty_name = format!("{}::{}", field.ty.to_token_stream(), UNMERGEABLE_NAME);
                    Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
                } else {
                    field.ty.clone()
                };

                field.vis = parse_quote! { pub };
                field.ty = parse_quote! { #mergeable_type };
            });
        }
    } else {
        unimplemented!()
    }
}

#[proc_macro_attribute]
pub fn mergeable(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut original_ast: DeriveInput = syn::parse(input).unwrap();

    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let global_options =
        FieldOptions::from_list(&attr_args).expect("Global options could not parse attr list");

    let field_options = extract_field_attrs(&mut original_ast);

    let mut mergeable_ast = original_ast.clone();
    let original_name = original_ast.ident.clone();

    let mergeable_name = syn::Ident::new(MERGEABLE_NAME, Span::call_site());
    let unmergeable_name = syn::Ident::new(UNMERGEABLE_NAME, Span::call_site());

    mergeable_ast.ident = mergeable_name.clone();
    original_ast.ident = unmergeable_name.clone();

    convert_fields_to_optional(&mut mergeable_ast, &field_options, &global_options);
    convert_nested_fields(&mut original_ast, &field_options, &global_options);

    let merge_fields = if let Data::Struct(s) = &original_ast.data {
        if let Fields::Named(named_fields) = &s.fields {
            named_fields.named.iter().map(|field| {
                let field_options = field_options.get_by_field(field);
                let name = field.clone().ident.unwrap();

                if field_options.is_nested {
                    quote! {
                        #name: merges::nested_merge(&self.#name, &rhs.#name)
                    }
                } else {
                    quote! {
                        #name: merges::primitive_merge(&self.#name, &rhs.#name)
                    }
                }
            })
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    let to_mergeable = if let Data::Struct(s) = &original_ast.data {
        if let Fields::Named(named_fields) = &s.fields {
            named_fields.named.iter().map(|field| {
                let field_options = field_options.get_by_field(field);
                let is_nested = field_options.is_nested;

                let name = field.clone().ident;

                if is_nested {
                    quote! {
                        #name: Some(orig.#name.into())
                    }
                } else {
                    quote! {
                        #name: Some(orig.#name)
                    }
                }
            })
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    let to_unwrapped = if let Data::Struct(s) = &original_ast.data {
        if let Fields::Named(named_fields) = &s.fields {
            named_fields.named.iter().map(|field| {
                let field_options = field_options.get_by_field(field);
                let is_nested = field_options.is_nested;
                let name = field.clone().ident;

                if is_nested {
                    quote! {
                        #name: orig.#name.unwrap().into()
                    }
                } else {
                    quote! {
                        #name: orig.#name.unwrap()
                    }
                }
            })
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    original_ast.vis = parse_quote! { pub };
    mergeable_ast.vis = parse_quote! { pub };

    let rename_as = LitStr::new(&original_name.to_string(), Span::call_site());

    if !global_options.skip_deserialize {
        original_ast.attrs.insert(
            0,
            parse_quote! {
                #[derive(Deserialize)]
            },
        );

        mergeable_ast.attrs.insert(
            0,
            parse_quote! {
                #[derive(Deserialize)]
            },
        );
    }

    mergeable_ast
        .attrs
        .push(parse_quote! { #[ts(rename = #rename_as)] });

    let token_stream = quote! {

        #[allow(non_snake_case)]
        pub mod #original_name {
            use super::*;
            use merges::Merges;
            use serde::Deserialize;

            #original_ast

            #[derive(Default)]
            #mergeable_ast

            impl From<#unmergeable_name> for #mergeable_name {
                fn from(orig: #unmergeable_name) -> Self {
                    Self {
                        #(#to_mergeable),*
                    }
                }
            }

            impl From<#mergeable_name> for #unmergeable_name {
                fn from(orig: #mergeable_name) -> Self {
                    Self {
                        #(#to_unwrapped),*
                    }
                }
            }

            impl Merges for #mergeable_name {
                fn merge(&self, rhs: &Self) -> Self {
                    Self {
                        #(#merge_fields),*
                    }
                }
            }
        }
    };

    token_stream.into()
}

struct AssociatedStruct {
    key_ident: Ident,
    source_struct: ItemStruct,
}

impl Parse for AssociatedStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_ident: Ident = input.parse()?;

        input.parse::<Token![=>]>()?;
        let content;

        braced!(content in input);
        let original_struct: ItemStruct = content.parse()?;
        input.parse::<Option<Token![,]>>()?;

        Ok(AssociatedStruct {
            key_ident: type_ident,
            source_struct: original_struct,
        })
    }
}

struct Mergeable {
    source_struct: ItemStruct,
    mergeable_struct: ItemStruct,
    unmergeable_struct: ItemStruct,
}

impl Parse for Mergeable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut associated_keys = HashMap::new();

        while !input.is_empty() {
            let s: AssociatedStruct = input.parse()?;
            let associated_key = s.key_ident.to_string();

            if !associated_keys.contains_key(&associated_key) {
                associated_keys.insert(associated_key, s.source_struct);
            } else {
                return Err(syn::Error::new_spanned(
                    s.key_ident,
                    format!("Associated key, {associated_key}, was already specified."),
                ));
            }
        }

        let required_keys = ["source", "mergeable", "unmergeable"];

        for key in required_keys {
            if !associated_keys.contains_key(key) {
                return Err(syn::Error::new(
                    input.span(),
                    format!("mergeable_fn! must have associated key, {key}. See docs for details."),
                ));
            }
        }

        Ok(Mergeable {
            source_struct: associated_keys.remove("source").unwrap(),
            mergeable_struct: associated_keys.remove("mergeable").unwrap(),
            unmergeable_struct: associated_keys.remove("unmergeable").unwrap(),
        })
    }
}

#[proc_macro]
pub fn mergeable_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Mergeable {
        source_struct,
        mut mergeable_struct,
        mut unmergeable_struct,
    } = parse_macro_input!(input as Mergeable);

    let mergeable_name = mergeable_struct.ident.clone();
    let unmergeable_name = unmergeable_struct.ident.clone();
    
    mergeable_struct.fields = source_struct.fields.clone();
    unmergeable_struct.fields = source_struct.fields.clone();

    let token_stream = quote! {

        #mergeable_struct

        #unmergeable_struct

        impl merges::HasMergeableVariant for #unmergeable_name {
            type MergeableType = #mergeable_name;
        }

        impl merges::HasUnmergeableVariant for #mergeable_name {
            type UnmergeableType = #unmergeable_name;
        }
    };

    token_stream.into()
}
