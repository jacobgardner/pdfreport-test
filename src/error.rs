use thiserror::Error;

use crate::units::MeasurementParseError;

#[derive(Error, Debug)]
pub enum BadPdfLayout {
    #[error("Could not find style, {style_name}, in stylesheet. Style names are case-sensitive.")]
    UnmatchedStyle { style_name: String },

    #[error("Unable to parse underlying pdf: {source}")]
    MeasurementParseError {
        #[from]
        source: MeasurementParseError,
    },

    #[error("Error computing the flex layout: {source}")]
    LayoutComputationError {
        #[from]
        source: stretch2::Error,
    },
}

// impl From<MeasurementParseError> for BadPdfLayout {
//     fn from(source: MeasurementParseError) -> Self {
//         todo!()
//     }
// }

// mod external_crate {
//     pub type Closure = Box<dyn Fn() -> usize>;
//     pub fn external_fn(_closure: Closure) {
//         // This is in another crate
//     }
// }

// type NestedFn<'a> = Box<dyn Fn(&'a str) -> external_crate::Closure>;

// pub struct Example<'a> {
//     ref1: &'a String,
//     nested_fn: NestedFn<'a>,
// }

// impl<'a> Example<'a> {
//     pub fn new(ref1: &'a String, nested_fn: NestedFn<'a>) -> Self {
//         Self { ref1, nested_fn }
//     }

//     pub fn do_something(&self) {
//         (self.nested_fn)(&self.ref1[1..]);
//     }
// }

// pub fn confusing() {
//     let outer_str = String::from("abcd");

//     {
//         let compute: NestedFn = Box::new(|s| {
//             Box::new(move || {
//                 s.len()
//             })
//         });

//         let e = Example::new(&outer_str, compute);
//     }
// }
