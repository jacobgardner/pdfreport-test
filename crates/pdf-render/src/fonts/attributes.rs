use num_derive::FromPrimitive;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontAttributes {
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontStyle, // Italic/Normal/Oblique
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

impl From<&str> for FontStyle {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "italic" => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl From<&str> for FontWeight {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "lighter" => FontWeight::Light,
            "bold" => FontWeight::Bold,
            "bolder" => FontWeight::ExtraBold,
            "normal" => FontWeight::Regular,
            other => {
                if let Ok(num) = other.parse::<u32>() {
                    num::FromPrimitive::from_u32(num).unwrap_or_default()
                } else {
                    FontWeight::Regular
                }
            }
        }
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Regular
    }
}
