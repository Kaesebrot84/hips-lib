#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn new() -> Self {
        Color::from_rgba(0, 0, 0, 0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn from_rgb_ut() {
        let r = 0;
        let g = 100;
        let b = 200;
        let color = Color::from_rgb(r, g, b);
        assert_eq!(r, color.r);
        assert_eq!(g, color.g);
        assert_eq!(b, color.b);
    }

    #[test]
    fn from_rgba_ut() {
        let r = 0;
        let g = 100;
        let b = 200;
        let a = 255;
        let color = Color::from_rgba(r, g, b, a);
        assert_eq!(r, color.r);
        assert_eq!(g, color.g);
        assert_eq!(b, color.b);
        assert_eq!(a, color.a);
    }

    #[test]
    fn new_ut() {
        let color = Color::new();
        assert_eq!(0, color.r);
        assert_eq!(0, color.g);
        assert_eq!(0, color.b);
        assert_eq!(0, color.a);
    }
}
