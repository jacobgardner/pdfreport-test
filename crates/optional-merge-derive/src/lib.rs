extern crate proc_macro;

mod associated_struct;
mod config;
mod field_options;

use associated_struct::Mergeable;
use field_options::{extract_field_attrs, FieldOptions, FieldsOptions};
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{
    self, parse_macro_input, parse_quote_spanned, spanned::Spanned, Fields, ItemStruct, Type,
};

fn convert_fields_to_optional(
    mergeable_struct: &mut ItemStruct,
    fields_options: &FieldsOptions,
    _global_options: &FieldOptions,
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

            let ty = parse_quote_spanned! { field.ty.span() => Option< #mergeable_type > };
            field.ty = ty;
        });
    } else {
        unimplemented!();
    }
}

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
    mergeable_struct
        .attrs
        .extend(source_struct.attrs.iter().cloned());
    unmergeable_struct.fields = source_struct.fields.clone();
    unmergeable_struct
        .attrs
        .extend(source_struct.attrs.iter().cloned());

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
