use std::collections::HashMap;

use syn::{parse::Parse, Ident, ItemStruct, Token, braced};

pub(crate) struct AssociatedStruct {
    pub(crate) key_ident: Ident,
    pub(crate) source_struct: ItemStruct,
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

pub(crate) struct Mergeable {
    pub(crate) source_struct: ItemStruct,
    pub(crate) mergeable_struct: ItemStruct,
    pub(crate) unmergeable_struct: ItemStruct,
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
            source_struct: associated_keys.remove("source").expect("We already have verified that this exists in the map."),
            mergeable_struct: associated_keys.remove("mergeable").expect("We already have verified that this exists in the map."),
            unmergeable_struct: associated_keys.remove("unmergeable").expect("We already have verified that this exists in the map."),
        })
    }
}
