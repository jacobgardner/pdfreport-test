
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

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Pt(pub f64);

impl From<f64> for Pt {
    fn from(pt: f64) -> Self {
        Self(pt)
    }
}

