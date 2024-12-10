pub mod ops;
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec3: [f64; 3],
}

impl Vec3 {
    pub fn new(arr: &[f64; 3]) -> Self {
        Self { vec3: *arr }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vec3[0] * other.vec3[0] + self.vec3[1] * other.vec3[1] + self.vec3[2] * other.vec3[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(&[
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> f64 {
        (self[0].powi(2) + self[1].powi(2) + self[3].powi(2)).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.length().powi(2)
    }
}

pub type Point = Vec3;
impl Point {
    pub fn x(&self) -> f64 {
        self.vec3[0]
    }
    pub fn y(&self) -> f64 {
        self.vec3[1]
    }
    pub fn z(&self) -> f64 {
        self.vec3[2]
    }
}
