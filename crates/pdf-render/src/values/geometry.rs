use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
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

#[derive(Debug, PartialEq)]
pub struct Mm(pub f64);

#[derive(Default, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Pt(pub f64);

impl From<f64> for Pt {
    fn from(pt: f64) -> Self {
        Self(pt)
    }
}

const MM_TO_PT: f64 = 2.8346456692913;

impl From<Mm> for Pt {
    fn from(mm: Mm) -> Self {
        Pt(mm.0 * MM_TO_PT)
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

#[derive(Debug, Clone)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}
