use serde::Deserialize;
use ts_rs::TS;

use crate::error::UserInputError;

#[derive(TS, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "&str")]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn white() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl From<Color> for printpdf::Color {
    fn from(color: Color) -> Self {
        Self::Rgb(printpdf::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
            icc_profile: None,
        })
    }
}

impl TryFrom<&str> for Color {
    type Error = UserInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        println!("Color: {value}");
        let (r, g, b, a) = color_processing::Color::new_string(value)?.get_rgba();

        Ok(Color { r, g, b, a })
    }
}
