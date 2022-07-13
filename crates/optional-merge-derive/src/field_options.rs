use std::collections::HashMap;

use darling::{FromField, FromMeta};
use syn::{Data, DeriveInput, Field, Fields, ItemStruct, Type};

use crate::config::FIELD_ATTR;

#[allow(dead_code)]
#[derive(Debug, FromField)]
#[darling(attributes(mergeable))]
pub struct MergeableField {
    ident: Option<syn::Ident>,
    ty: Type,
    #[darling(default)]
    nested: bool,
}

#[derive(Clone, FromMeta, Debug, Default)]
#[darling(default)]
pub struct FieldOptions {
    pub is_nested: bool,
}

impl From<MergeableField> for FieldOptions {
    fn from(field: MergeableField) -> Self {
        Self {
            is_nested: field.nested,
        }
    }
}

#[derive(Debug, Default)]
pub struct FieldsOptions(HashMap<String, FieldOptions>);

impl FieldsOptions {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert_by_field(&mut self, field: &Field) {
        let mergeable_field = MergeableField::from_field(field).unwrap();

        self.insert(
            field
                .ident
                .clone()
                .expect("Expected named field to have ident")
                .to_string(),
            mergeable_field.into(),
        );
    }

    pub fn insert(&mut self, field_name: String, options: FieldOptions) {
        self.0.insert(field_name, options);
    }

    pub fn get_by_field(&self, field: &Field) -> FieldOptions {
        let field_name = field
            .ident
            .clone()
            .expect("Expected named field to have name")
            .to_string();

        self.get(&field_name)
    }

    pub fn get(&self, field_name: &str) -> FieldOptions {
        self.0.get(field_name).cloned().unwrap_or_default()
    }
}

pub fn extract_field_attrs(unmergeable_struct: &mut ItemStruct) -> FieldsOptions {
    let mut field_options = FieldsOptions::new();

    if let Fields::Named(named_fields) = &mut unmergeable_struct.fields {
        for field in named_fields.named.iter_mut() {
            let mergeable_attr_index = field
                .attrs
                .iter()
                .position(|attr| attr.path.is_ident(FIELD_ATTR));

            if let Some(index) = mergeable_attr_index {
                field_options.insert_by_field(field);

                field.attrs.remove(index);
            }
        }
    } else {
        unimplemented!();
    }

    field_options
}
