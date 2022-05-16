#[derive(Debug, Clone)]
pub struct BlockSpacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BlockSpacing {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    #[allow(dead_code)]
    pub fn width(&self) -> f32 {
        self.left + self.right
    }

    #[allow(dead_code)]
    pub fn height(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for BlockSpacing {
    fn default() -> Self {
        Self::new(0., 0., 0., 0.)
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_calc() {
        let spacing = BlockSpacing::new(4., 5., 6., 7.);
        assert_eq!(spacing.width(), 12.);
    }
}
