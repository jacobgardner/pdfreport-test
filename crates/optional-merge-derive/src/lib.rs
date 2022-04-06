extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, Type};

// #[proc_macro_derive(MergeOptional)]
#[proc_macro_attribute]
pub fn mergable(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();


    if let Data::Struct(s) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut s.fields {
            named_fields.named.iter_mut().for_each(|f| {
                let ty = f.ty.clone();
                f.ty = Type::Verbatim(quote! { Option<#ty> });
            });
            // named_fields.   
        }

    } else {
        unimplemented!()
    }

    let original_ast = ast.clone();


    let merge_fields = if let Data::Struct(s) = &original_ast.data {
        if let Fields::Named(named_fields) = &s.fields {

            named_fields.named.iter().map(|field| {
                let name = field.clone().ident;

                quote! {
                    #name: self.#name.merge(&rhs.#name)
                }
            })

            // vec![quote! {
                
            // }]
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    // syn::

    // let expanded = quote! {
    //     impl Merges for #name {

    //     }
    // };

    // TokenStream::from(expanded)
    let name = original_ast.ident.clone();

    quote! {
        #ast

        impl Merges for #name {
            fn merge(&self, rhs: &Self) -> Self {
                Self {
                    #(#merge_fields),*
                }
            }
        }

        // impl Deref for #name {

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
