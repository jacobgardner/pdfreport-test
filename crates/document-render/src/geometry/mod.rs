
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> From<(T, T)> for Size<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

pub struct Mm(pub f64);
