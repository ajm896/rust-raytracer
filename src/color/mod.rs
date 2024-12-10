use std::io::{self, Write};

use crate::vector::Vec3;
pub type Color = Vec3;
impl Color {
    pub fn write_color(&self) {
        println!("{} {} {}", self.r(), self.g(), self.b());
    }

    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn b(&self) -> f64 {
        self[2]
    }
}
