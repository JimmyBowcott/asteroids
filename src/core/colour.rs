#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
}

impl From<RGB> for sdl2::pixels::Color {
    fn from(c: RGB) -> Self {
        sdl2::pixels::Color::RGB(c.r, c.g, c.b)
    }
}
