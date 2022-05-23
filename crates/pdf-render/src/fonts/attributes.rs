use num_derive::FromPrimitive;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Deserialize, Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontAttributes {
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontSlant, // Italic/Normal/Oblique
}

impl FontAttributes {
    pub fn italic() -> Self {
        Self {
            style: FontSlant::Italic,
            ..Default::default()
        }
    }

    pub fn bold() -> Self {
        Self {
            weight: FontWeight::Bold,
            ..Default::default()
        }
    }
}

#[derive(TS, Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
#[ts(export)]
pub enum FontSlant {
    Normal,
    Italic,
}

impl Default for FontSlant {
    fn default() -> Self {
        FontSlant::Normal
    }
}

impl From<&str> for FontSlant {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "italic" => FontSlant::Italic,
            _ => FontSlant::Normal,
        }
    }
}

#[derive(TS, Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
#[ts(export)]
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
