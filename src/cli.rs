use clap::Parser;
use std::fs::{self, File};

use pdf_render::{build_pdf_from_dom, doc_structure::DocStructure};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(default_value_t = String::from("./assets/example.json"))]
    file_path: String,
}

pub fn main() {
    let args = Args::parse();

    let example_json = fs::read_to_string(&args.file_path).unwrap();

    let pdf_dom: DocStructure = serde_json::from_str(&example_json).unwrap();

    let mut file_to_write = File::create("output.pdf").unwrap();
    build_pdf_from_dom(&pdf_dom, &mut file_to_write).unwrap();
}
