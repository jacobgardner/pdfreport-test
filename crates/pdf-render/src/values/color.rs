use crate::error::UserInputError;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
        }
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
        let (r, g, b, a) = color_processing::Color::new_string(value)?.get_rgba();

        Ok(Color { r, g, b })
    }
}
