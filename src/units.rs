use lazy_static::lazy_static;
use printpdf::{Mm, Pt};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnitParseError {
    #[error(
        "Malformed source string, \"{source_str}\", expected a number or a number followed by a unit."
    )]
    MalformedSource { source_str: String },
    #[error("Unsupported unit type, \"{attached_unit}\"")]
    UnsupportedUnit { attached_unit: String },
    #[error("Quantity, \"{quantity_str}\", could not be parsed into a float.")]
    UnparsableQuantity { quantity_str: String },
}

pub fn unit_to_pt(svg_unit: &str) -> Result<Pt, UnitParseError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?i)(?P<quantity>[\.\d]+)(?P<units>\D+)?$")
            .expect("Regex should have been tested before production");
    }

    let capture_groups = RE
        .captures(svg_unit)
        .ok_or_else(|| UnitParseError::MalformedSource {
            source_str: String::from(svg_unit),
        })?;
    let quantity: f64 = capture_groups
        .name("quantity")
        .expect("Since the regex passed, we should have a quantity group.")
        .as_str()
        .parse()
        .map_err(|_| UnitParseError::UnparsableQuantity {
            quantity_str: String::from(capture_groups.name("quantity").unwrap().as_str()),
        })?;
    let units = capture_groups.name("units").map_or("px", |u| u.as_str());

    Ok(match units.to_lowercase().as_str() {
        "px" => Mm(quantity * (25.4 / 300.)).into(),
        "mm" => Mm(quantity).into(),
        "cm" => Mm(quantity * 10.0).into(),
        "pt" => Pt(quantity),
        "in" => Pt(quantity * 72.),
        "pc" => Pt(quantity * 6.),
        unit => {
            return Err(UnitParseError::UnsupportedUnit {
                attached_unit: String::from(unit),
            })
        }
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_to_pt_px() -> Result<(), UnitParseError> {
        // pixels * 72 / DPI = pt
        assert_eq!(unit_to_pt("50")?, Pt(12.0));
        assert_eq!(unit_to_pt("50px")?, Pt(12.0));

        assert_eq!(unit_to_pt("100")?, Pt(24.0));
        assert_eq!(unit_to_pt("100px")?, Pt(24.0));

        Ok(())
    }

    #[test]
    fn test_svg_to_pt_mm() -> Result<(), UnitParseError> {
        // Taken from a lookup table
        assert_eq!(unit_to_pt("50mm")?, Pt(141.7322834646));
        assert_eq!(unit_to_pt("20mm")?, Pt(56.6929133858));

        Ok(())
    }

    #[test]
    fn test_svg_to_pt_cm() -> Result<(), UnitParseError> {
        // Taken from a lookup table
        assert_eq!(unit_to_pt("5cm")?, Pt(141.7322834646));
        assert_eq!(unit_to_pt("2cm")?, Pt(56.6929133858));

        Ok(())
    }

    #[test]
    fn test_svg_to_pt_pt() -> Result<(), UnitParseError> {
        // 1:1
        assert_eq!(unit_to_pt("5pt")?, Pt(5.));
        assert_eq!(unit_to_pt("2pt")?, Pt(2.));
        assert_eq!(unit_to_pt("12pt")?, Pt(12.));
        assert_eq!(unit_to_pt("12.5pt")?, Pt(12.5));

        Ok(())
    }

    #[test]
    fn test_svg_to_pt_in() -> Result<(), UnitParseError> {
        // 1:72
        assert_eq!(unit_to_pt("5in")?, Pt(360.));
        assert_eq!(unit_to_pt("2in")?, Pt(144.));
        assert_eq!(unit_to_pt("12in")?, Pt(864.));

        Ok(())
    }

    #[test]
    fn test_svg_to_pt_pc() -> Result<(), UnitParseError> {
        // 1:6
        assert_eq!(unit_to_pt("5pc")?, Pt(30.));
        assert_eq!(unit_to_pt("2pc")?, Pt(12.));
        assert_eq!(unit_to_pt("12pc")?, Pt(72.));

        Ok(())
    }

    #[test]
    fn test_unsupported_unit() -> Result<(), UnitParseError> {
        assert!(matches!(
            unit_to_pt("5rem"),
            Err(UnitParseError::UnsupportedUnit { .. })
        ));

        assert!(matches!(
            unit_to_pt("5.5.5px"),
            Err(UnitParseError::UnparsableQuantity { .. })
        ));

        assert!(matches!(
            unit_to_pt("px"),
            Err(UnitParseError::MalformedSource { .. })
        ));

        Ok(())
    }
}
