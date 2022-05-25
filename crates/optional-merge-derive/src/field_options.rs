use std::collections::HashMap;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Paren, Attribute,
    AttributeArgs, Data, DeriveInput, Expr, ExprParen, ExprTuple, Field, Fields, Token, Type,
};

use crate::config::FIELD_ATTR;

#[derive(Debug, FromField)]
#[darling(attributes(mergeable))]
pub struct MergeableField {
  ident: Option<Ident>,
  ty: Type,
  #[darling(default)]
  nested: bool,  
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mergeable), supports(struct_named))]
pub struct MergeableStruct {
  ident: Ident,
  data: darling::ast::Data<darling::util::Ignored, MergeableField>,
  #[darling(default)] 
  use_null_in_serde: bool,
  
}

#[derive(Clone, FromMeta, Debug)]
#[darling(default)]
pub struct FieldOptions {
    pub rename: Option<String>,
    pub use_null_in_serde: bool,
    pub is_nested: bool,
}

#[derive(Debug)]
pub struct FieldsOptions(HashMap<String, FieldOptions>);

impl FieldsOptions {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert_by_attr(&mut self, field: &Field, attribute: Attribute) {
        let tokens = attribute.to_token_stream().into();

        let mut args = None;

        // FIXME: Hack to get around parse_macro_input!
        let parse_stuff = |args: &mut Option<AttributeArgs>| {
          let new_args = parse_macro_input!(tokens as AttributeArgs);
          
          args.replace(new_args);
          
          TokenStream::new()
        };
        
        let result = parse_stuff(&mut args);
        println!("Parse result: {:?}", result);
        
        println!("A");
        let args = args.unwrap();
        
        FieldOptions::from
        
        let options = FieldOptions::from_list(&args).unwrap();
        println!("B");

        self.insert(
            field
                .ident
                .clone()
                .expect("Expected named field to have ident")
                .to_string(),
            options,
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

impl Default for FieldOptions {
    fn default() -> Self {
        Self {
            rename: None,
            use_null_in_serde: false,
            is_nested: false,
        }
    }
}

// impl Parse for FieldOptions {
//     fn parse(mut input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let mut options = FieldOptions::default();

//         println!("Input: {:?}", input);
//         if input.peek(Paren) {
//             let p = input.parse::<syn::Type>().unwrap();
//             println!("P: {:?}", p);
//         }

//         let expression_iter = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
//         println!("expr_iter: {:?}", expression_iter);

//         println!("----------------------------");
//         for thing in expression_iter {
//           println!("Thing: {:?}", thing);
//             match thing {
//                 Expr::Assign(assignment) => {
//                     let option_name = if let Expr::Path(path) = assignment.left.as_ref() {
//                         path.path
//                             .get_ident()
//                             .expect("Expected left side to have named path")
//                             .to_string()
//                     } else {
//                         unreachable!()
//                     };

//                     println!("Option: {option_name}");
//                 }
//                 Expr::Path(standalone) => {
//                     let ident = standalone
//                         .path
//                         .get_ident()
//                         .expect("Expected standalone to have named path")
//                         .to_string();
//                     println!("Option: {ident}");

//                     match ident.as_str() {
//                         "nested" => {
//                             options.is_nested = true;
//                         }
//                         "use_null_in_serde" => {
//                             options.use_null_in_serde = true;
//                         }
//                         _ => {}
//                     }
//                 }
//                 _ => unreachable!(),
//             }
//         }

//         Ok(options)
//     }
// }

pub fn extract_field_attrs(ast: &mut DeriveInput) -> FieldsOptions {
    let mut field_options = FieldsOptions::new();

    if let Data::Struct(unmergeable_struct) = &mut ast.data {
        if let Fields::Named(named_fields) = &mut unmergeable_struct.fields {
            for field in named_fields.named.iter_mut() {
                let mergeable_attr_index = field
                    .attrs
                    .iter()
                    .position(|attr| attr.path.is_ident(FIELD_ATTR));

                if let Some(index) = mergeable_attr_index {
                    let field_attr = field.attrs.remove(index);

                    field_options.insert_by_attr(&field, field_attr);
                }
            }
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }

    field_options
}
