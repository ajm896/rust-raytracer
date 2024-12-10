use crate::vector::Vec3;
pub type Color = Vec3;
impl Color {
    pub fn write_color(&self) {
        println!("{} {} {}", self.r() as u8, self.g() as u8, self.b() as u8);
    }

    pub fn r(&self) -> f64 {
        self[0] * 255.
    }

    pub fn g(&self) -> f64 {
        self[1] * 255.
    }

    pub fn b(&self) -> f64 {
        self[2] * 255.
    }

    pub fn colors(preset: Presets) -> Color {
        match preset {
            Presets::White => Color::new(1.0, 1.0, 1.0),
            Presets::Black => Color::new(0.0, 0.0, 0.0),
            Presets::Red => Color::new(1.0, 0.0, 0.0),
            Presets::Green => Color::new(0.0, 1.0, 0.0),
            Presets::Blue => Color::new(0.0, 0.0, 1.0),
        }
    }
}

pub enum Presets {
    White,
    Black,
    Red,
    Green,
    Blue,
}
