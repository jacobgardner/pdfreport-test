use std::{
    fmt::Display,
    ops::{Add, Sub, AddAssign},
};

#[derive(Debug, Clone)]
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

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pt.", self.0)
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

#[derive(Debug, Clone)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}
