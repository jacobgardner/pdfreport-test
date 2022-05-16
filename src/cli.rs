use std::fs::{self, File};

use serde_json;

use pdf_render::{build_pdf_from_dom, dom::PdfDom};

pub fn main() {
    let example_json = fs::read_to_string("./assets/example.json").unwrap();

    let pdf_dom: PdfDom = serde_json::from_str(&example_json).unwrap();

    let mut file_to_write = File::create("output.pdf").unwrap();
    build_pdf_from_dom(&pdf_dom, &mut file_to_write).unwrap();
}
