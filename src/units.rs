use lazy_static::lazy_static;
use printpdf::{Mm, Pt};
use regex::Regex;

// TODO: Return Result instead of unwrapping everywhere

pub fn unit_to_pt(svg_unit: &str) -> Pt {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?i)(?P<quantity>[\.\d]+)(?P<units>\D+)?$").unwrap();
    }

    let capture_groups = RE.captures(svg_unit).unwrap();
    let quantity: f64 = capture_groups
        .name("quantity")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let units = capture_groups.name("units").map_or("px", |u| u.as_str());

    match units.to_lowercase().as_str() {
        "px" => Mm(quantity * (25.4 / 300.)).into(),
        "mm" => Mm(quantity).into(),
        "cm" => Mm(quantity * 10.0).into(),
        "pt" => Pt(quantity),
        "in" => Pt(quantity * 72.),
        "pc" => Pt(quantity * 6.),
        _ => panic!("Unknown unit types {units}"),
    }
}

pub fn percent_to_num(percent: &str) -> f64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?i)(?P<quantity>[\.\d]+)%$").unwrap();
    }

    let capture_groups = RE.captures(percent).unwrap();
    let quantity: f64 = capture_groups
        .name("quantity")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    return quantity;
}
