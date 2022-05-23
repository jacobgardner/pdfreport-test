extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{self, Attribute, Data, Fields, Type, Token};

#[proc_macro_derive(MergeOptional, attributes(nested))]
pub fn mergeable(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    let original_ast = ast.clone();

    if let Data::Struct(s) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut s.fields {
            named_fields.named.iter_mut().for_each(|field| {
                let nested_pos = field.attrs.iter().position(|a| a.path.is_ident("nested"));
                // let ty = f.ty.clone();
                let mergeable_type: syn::Type = if let Some(pos) = nested_pos {
                    field.attrs.remove(pos);
                    let ty_name = format!("Mergeable{}", field.ty.to_token_stream());
                    Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
                } else {
                    field.ty.clone()
                };

                
                let s: syn::ItemStruct = syn::parse2(quote! {
                    struct test {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        field: Option<String>
                    }
                }).unwrap();
                
                let skip_optional_attribute = if let syn::Fields::Named(fields) = s.fields {
                    fields.named[0].attrs[0].clone()
                } else {
                    unreachable!();
                };
                // let field: syn::Expr = syn::parse2(optional_attribute).unwrap();
                // field.attrs.push()

                // // let p: syn::parse::ParseStream = syn::parse2(optional_attribute.into()).unwrap();
                
                // let attrib = syn::Attribute {
                //     pound_token: syn::token::Pound::default(),
                //     style: syn::AttrStyle::Outer,
                //     bracket_token: syn::token::Bracket::default(), 
                // };
                // // let a = p.call(syn::Attribute::parse_outer).unwrap();
                // // syn::Attribute::parse_outer(p)
                // // syn::parse::ParseStream::
                // // let a = syn::Attribute {
                // //     pound_token: syn::Pound
                // // };

                field.attrs.push(skip_optional_attribute);
                field.ty = Type::Verbatim(quote! { Option< #mergeable_type > });
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
    // let quoted_name = format!("\"{name}\"");
    let rename_str = proc_macro2::TokenStream::from_str(&format!("rename=\"{name}\"")).unwrap();
    let mergeable_name = syn::Ident::new(&format!("Mergeable{}", name), Span::call_site());
    ast.ident = mergeable_name.clone();

    let token_stream: TokenStream = quote! {
        #[derive(TS, PartialEq, Deserialize, Clone, Debug, Default)]
        #[ts(export, #rename_str)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
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
