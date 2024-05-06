
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub const fn color(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color {
        r,
        g,
        b,
        a
    }
}

impl Color {

    pub const BLACK: Color = color(0.0, 0.0, 0.0, 1.0);

    pub fn from_hex(hex: u32) -> Color {
        let bytes = hex.to_be_bytes();
        color(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
            bytes[3] as f32 / 255.0
        )
    }

    pub fn lerp(&self, other: Color, t: f32) -> Self {
        Self {
            r: self.r * (1.0 - t) + other.r * t,
            g: self.g * (1.0 - t) + other.g * t,
            b: self.b * (1.0 - t) + other.b * t,
            a: self.a * (1.0 - t) + other.a * t
        }
    }

    pub fn darken(&self, t: f32) -> Self {
        self.lerp(Self::BLACK, t)
    }

}
