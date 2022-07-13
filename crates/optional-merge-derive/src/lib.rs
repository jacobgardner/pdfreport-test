extern crate proc_macro;

mod associated_struct;
mod config;
mod field_options;
mod output;

use associated_struct::Mergeable;
use config::{MERGEABLE_NAME, UNMERGEABLE_NAME};
use darling::FromMeta;
use field_options::{extract_field_attrs, FieldOptions, FieldsOptions};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{
    self, parse_macro_input, parse_quote, Attribute, AttributeArgs, Data, DeriveInput, Fields,
    ItemStruct, LitStr, Type,
};

fn build_skip_optional_attr() -> Attribute {
    parse_quote! { #[serde(skip_serializing_if = "Option::is_none")] }
}

fn convert_fields_to_optional(
    mergeable_struct: &mut ItemStruct,
    fields_options: &FieldsOptions,
    global_options: &FieldOptions,
) {
    if let Fields::Named(named_fields) = &mut mergeable_struct.fields {
        named_fields.named.iter_mut().for_each(|field| {
            let field_options = fields_options.get_by_field(field);

            let mergeable_type = if field_options.is_nested {
                let ty_name = format!(
                    "<{} as merges::HasMergeableVariant>::MergeableType",
                    field.ty.to_token_stream()
                );
                Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
            } else {
                field.ty.clone()
            };

            field.ty = parse_quote! { Option< #mergeable_type > };
        });
    } else {
        unimplemented!();
    }
}

// #[proc_macro_attribute]
// pub fn mergeable(
//     attr: proc_macro::TokenStream,
//     input: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     let mut original_ast: DeriveInput = syn::parse(input).unwrap();

//     let attr_args = parse_macro_input!(attr as AttributeArgs);
//     let global_options =
//         FieldOptions::from_list(&attr_args).expect("Global options could not parse attr list");

//     // let field_options = extract_field_attrs(&mut original_ast);

//     let mut mergeable_ast = original_ast.clone();
//     let original_name = original_ast.ident.clone();

//     let mergeable_name = syn::Ident::new(MERGEABLE_NAME, Span::call_site());
//     let unmergeable_name = syn::Ident::new(UNMERGEABLE_NAME, Span::call_site());

//     mergeable_ast.ident = mergeable_name.clone();
//     original_ast.ident = unmergeable_name.clone();

//     convert_fields_to_optional(&mut mergeable_ast, &field_options, &global_options);
//     convert_nested_fields(&mut original_ast, &field_options, &global_options);

//     let merge_fields = if let Data::Struct(s) = &original_ast.data {
//         if let Fields::Named(named_fields) = &s.fields {
//             named_fields.named.iter().map(|field| {
//                 let field_options = field_options.get_by_field(field);
//                 let name = field.clone().ident.unwrap();

//                 if field_options.is_nested {
//                     quote! {
//                         #name: merges::nested_merge(&self.#name, &rhs.#name)
//                     }
//                 } else {
//                     quote! {
//                         #name: merges::primitive_merge(&self.#name, &rhs.#name)
//                     }
//                 }
//             })
//         } else {
//             unimplemented!()
//         }
//     } else {
//         unimplemented!()
//     };

//     let to_mergeable = if let Data::Struct(s) = &original_ast.data {
//         if let Fields::Named(named_fields) = &s.fields {
//             named_fields.named.iter().map(|field| {
//                 let field_options = field_options.get_by_field(field);
//                 let is_nested = field_options.is_nested;

//                 let name = field.clone().ident;

//                 if is_nested {
//                     quote! {
//                         #name: Some(orig.#name.into())
//                     }
//                 } else {
//                     quote! {
//                         #name: Some(orig.#name)
//                     }
//                 }
//             })
//         } else {
//             unimplemented!()
//         }
//     } else {
//         unimplemented!()
//     };

//     let to_unwrapped = if let Data::Struct(s) = &original_ast.data {
//         if let Fields::Named(named_fields) = &s.fields {
//             named_fields.named.iter().map(|field| {
//                 let field_options = field_options.get_by_field(field);
//                 let is_nested = field_options.is_nested;
//                 let name = field.clone().ident;

//                 if is_nested {
//                     quote! {
//                         #name: orig.#name.unwrap().into()
//                     }
//                 } else {
//                     quote! {
//                         #name: orig.#name.unwrap()
//                     }
//                 }
//             })
//         } else {
//             unimplemented!()
//         }
//     } else {
//         unimplemented!()
//     };

//     original_ast.vis = parse_quote! { pub };
//     mergeable_ast.vis = parse_quote! { pub };

//     let rename_as = LitStr::new(&original_name.to_string(), Span::call_site());

//     if !global_options.skip_deserialize {
//         original_ast.attrs.insert(
//             0,
//             parse_quote! {
//                 #[derive(Deserialize)]
//             },
//         );

//         mergeable_ast.attrs.insert(
//             0,
//             parse_quote! {
//                 #[derive(Deserialize)]
//             },
//         );
//     }

//     mergeable_ast
//         .attrs
//         .push(parse_quote! { #[ts(rename = #rename_as)] });

//     let token_stream = quote! {

//         #[allow(non_snake_case)]
//         pub mod #original_name {
//             use super::*;
//             use merges::Merges;
//             use serde::Deserialize;

//             #original_ast

//             #[derive(Default)]
//             #mergeable_ast

//             impl From<#unmergeable_name> for #mergeable_name {
//                 fn from(orig: #unmergeable_name) -> Self {
//                     Self {
//                         #(#to_mergeable),*
//                     }
//                 }
//             }

//             impl From<#mergeable_name> for #unmergeable_name {
//                 fn from(orig: #mergeable_name) -> Self {
//                     Self {
//                         #(#to_unwrapped),*
//                     }
//                 }
//             }

//             impl Merges for #mergeable_name {
//                 fn merge(&self, rhs: &Self) -> Self {
//                     Self {
//                         #(#merge_fields),*
//                     }
//                 }
//             }
//         }
//     };

//     token_stream.into()
// }

#[proc_macro]
pub fn mergeable_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let global_options = FieldOptions { is_nested: false };

    let Mergeable {
        mut source_struct,
        mut mergeable_struct,
        mut unmergeable_struct,
    } = parse_macro_input!(input as Mergeable);

    let mergeable_name = mergeable_struct.ident.clone();
    let unmergeable_name = unmergeable_struct.ident.clone();

    let field_options = extract_field_attrs(&mut source_struct);

    mergeable_struct.fields = source_struct.fields.clone();
    unmergeable_struct.fields = source_struct.fields.clone();

    convert_fields_to_optional(&mut mergeable_struct, &field_options, &global_options);
    // convert_nested_fields(&mut unmergeable_struct, &field_options, &global_options);
    //

    let merge_fields = if let Fields::Named(named_fields) = &source_struct.fields {
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
    };

    let to_mergeable = if let Fields::Named(named_fields) = &source_struct.fields {
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
    };

    let to_unwrapped = if let Fields::Named(named_fields) = &source_struct.fields {
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
    };

    let token_stream = quote! {

        #mergeable_struct

        #unmergeable_struct

        impl merges::HasMergeableVariant for #unmergeable_name {
            type MergeableType = #mergeable_name;
        }

        impl merges::HasUnmergeableVariant for #mergeable_name {
            type UnmergeableType = #unmergeable_name;
        }
        
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

        impl merges::Merges for #mergeable_name {
            fn merge(&self, rhs: &Self) -> Self {
                Self {
                    #(#merge_fields),*
                }
            }
        }

    };

    token_stream.into()
}
