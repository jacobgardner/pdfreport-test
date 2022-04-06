extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{self, Data, Fields, Type};

#[proc_macro_derive(MergeOptional, attributes(nested))]
pub fn mergeable(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    let original_ast = ast.clone();

    if let Data::Struct(s) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut s.fields {
            named_fields.named.iter_mut().for_each(|f| {
                let nested_pos = f.attrs.iter().position(|a| a.path.is_ident("nested"));
                // let ty = f.ty.clone();
                let mergeable_type: syn::Type = if let Some(pos) = nested_pos {
                    f.attrs.remove(pos);
                    let ty_name = format!("Mergeable{}", f.ty.to_token_stream());
                    Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
                } else {
                    f.ty.clone()
                };

                f.ty = Type::Verbatim(quote! { Option< #mergeable_type > });
            });
        }
    } else {
        unimplemented!()
    }

    let merge_fields = if let Data::Struct(s) = &original_ast.data {
        if let Fields::Named(named_fields) = &s.fields {
            named_fields.named.iter().map(|field| {
                let name = field.clone().ident;

                quote! {
                    #name: self.#name.merge(&rhs.#name)
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
                let is_nested = field.attrs.iter().any(|a| a.path.is_ident("nested"));
                let name = field.clone().ident;

                // println!("N: {}", name);

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
                let is_nested = field.attrs.iter().any(|a| a.path.is_ident("nested"));
                let name = field.clone().ident;

                // println!("N: {}", name);

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

    let name = original_ast.ident.clone();
    let mergeable_name = syn::Ident::new(&format!("Mergeable{}", name), Span::call_site());
    ast.ident = mergeable_name.clone();

    let token_stream: TokenStream = quote! {
        #[derive(Deserialize, Clone)]
        #[serde(rename_all = "camelCase")]
        #ast

        impl From<#name> for #mergeable_name {
            fn from(orig: #name) -> #mergeable_name {
                Self {
                    #(#to_mergeable),*
                }
            }
        }

        impl From<#mergeable_name> for #name {
            fn from(orig: #mergeable_name) -> #name {
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
    .into();

    // println!("{}", token_stream.to_string());

    token_stream
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
