use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use regex::Regex;
use serde::Deserialize;

use lazy_static::lazy_static;

use crate::error::{DocumentGenerationError, UserInputError};

#[derive(Debug, Clone, Deserialize)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl From<Size<Mm>> for Size<Pt> {
    fn from(mm: Size<Mm>) -> Self {
        Self {
            width: mm.width.into(),
            height: mm.height.into(),
        }
    }
}

impl From<Size<Pt>> for Size<Mm> {
    fn from(pt: Size<Pt>) -> Self {
        Self {
            width: pt.width.into(),
            height: pt.height.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Size<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Mm(pub f64);

#[derive(Default, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Pt(pub f64);

#[derive(Deserialize)]
#[serde(untagged)]
enum ValueType {
    Num(f64),
    String(String),
}

impl<'de> Deserialize<'de> for Pt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: ValueType = Deserialize::deserialize(deserializer)?;

        match v {
            ValueType::Num(num) => Ok(Pt(num)),
            ValueType::String(str) => Ok(Pt::try_from(str.as_str())
                .map_err(|err| serde::de::Error::custom(format!("{}", err)))?),
        }
    }
}

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pt.", self.0)
    }
}

impl TryFrom<&str> for Pt {
    type Error = DocumentGenerationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?i)(?P<quantity>[\.\d]+)(?P<units>\D+)?$")
                .expect("Regex should have been tested before production");
        }

        let capture_groups =
            RE.captures(value)
                .ok_or_else(|| UserInputError::MalformedUnitString {
                    source_str: String::from(value),
                })?;
        let quantity: f64 = capture_groups
            .name("quantity")
            .expect("Since the regex passed, we should have a quantity group.")
            .as_str()
            .parse()
            .map_err(|_| UserInputError::UnparsableUnitQuantity {
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
                return Err(UserInputError::UnsupportedUnit {
                    attached_unit: String::from(unit),
                }
                .into());
            }
        })
    }
}

impl From<f64> for Pt {
    fn from(pt: f64) -> Self {
        Self(pt)
    }
}

const MM_TO_PT: f64 = 2.8346456692913;

impl From<Mm> for Pt {
    fn from(mm: Mm) -> Self {
        Self(mm.0 * MM_TO_PT)
    }
}

impl From<Pt> for Mm {
    fn from(pt: Pt) -> Self {
        Self(pt.0 * 1. / MM_TO_PT)
    }
}

impl Sub for Pt {
    type Output = Pt;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt(self.0 - rhs.0)
    }
}

impl Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self::Output {
        Pt(self.0 + rhs.0)
    }
}

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Pt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}
