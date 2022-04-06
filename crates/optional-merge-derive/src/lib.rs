extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{self, Data, Fields, Type};
use std::str::FromStr;

// #[proc_macro_attribute]
// pub fn nested(_args: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }

#[proc_macro_derive(MergeOptional, attributes(nested))]
pub fn mergable(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let Data::Struct(s) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut s.fields {
            named_fields.named.iter_mut().for_each(|f| {
                let nested_pos = f.attrs.iter().position(|a| a.path.is_ident("nested"));
                // let ty = f.ty.clone();
                let mergable_type: syn::Type = if let Some(pos) = nested_pos {
                    f.attrs.remove(pos);
                    let ty_name = format!("Mergable{}", f.ty.to_token_stream());
                    Type::Verbatim(proc_macro2::TokenStream::from_str(&ty_name).unwrap())
                } else {
                    f.ty.clone()
                };

                // let mergable_name = Type::Verbatim(quote! { #mergeable_name_string });
                // println!("{}", mergeable_name_string);

                f.ty = Type::Verbatim(quote! { Option< #mergable_type > });
            });
            // named_fields.
        }
    } else {
        unimplemented!()
    }

    let original_ast = ast.clone();

    let merge_fields = if let Data::Struct(s) = &ast.data {
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

    // ast.vis = syn::parse2(quote! { pub(super ) }).unwrap();

    // syn::

    // let expanded = quote! {
    //     impl Merges for #name {

    //     }
    // };

    // TokenStream::from(expanded)
    let name = original_ast.ident.clone();
    let mergable_name = format!("Mergable{}", name);
    // ast.ident = syn::parse2( quote! { #mergable_name }).unwrap();
    ast.ident = syn::Ident::new(&mergable_name, Span::call_site());

    quote! {

        #[derive(Deserialize, Clone)]
        #ast

            // impl Merges for #mergable_name {
            //     fn merge(&self, rhs: &Self) -> Self {
            //         Self {
            //             #(#merge_fields),*
            //         }
            //     }
            // }

    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
